use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::path::Path;
use std::ptr::NonNull;

use anyhow::{Context as _, Result, anyhow, ensure};
use block2::StackBlock;
use objc2::AnyThread as _;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2_core_ml::{
    MLComputeUnits, MLDictionaryFeatureProvider, MLFeatureProvider, MLFeatureValue, MLModel,
    MLModelConfiguration, MLMultiArray, MLMultiArrayDataType, MLPredictionOptions,
};
use objc2_foundation::{NSArray, NSDictionary, NSError, NSInteger, NSNumber, NSString, NSURL};

pub use objc2_core_ml::MLComputeUnits as ComputeUnits;

fn describe(error: &NSError) -> String {
    error.localizedDescription().to_string()
}

pub struct Model {
    inner: Retained<MLModel>,
    options: Retained<MLPredictionOptions>,
}

impl Model {
    pub fn load(path: &Path, units: MLComputeUnits) -> Result<Self> {
        let url = NSURL::from_file_path(path)
            .with_context(|| format!("invalid model path {}", path.display()))?;
        // SAFETY: the configuration is a fresh object owned here and complete before CoreML reads
        // it; `url` is a file URL, and CoreML reports a missing or invalid bundle as an error.
        let inner = unsafe {
            let config = MLModelConfiguration::new();
            config.setComputeUnits(units);
            MLModel::modelWithContentsOfURL_configuration_error(&url, &config)
        }
        .map_err(|error| anyhow!("loading {}: {}", path.display(), describe(&error)))?;
        // SAFETY: default prediction options.
        let options = unsafe { MLPredictionOptions::new() };
        Ok(Self { inner, options })
    }

    pub fn predict(&self, inputs: &[(&str, &MLMultiArray)]) -> Result<Outputs> {
        let keys: Vec<Retained<NSString>> = inputs
            .iter()
            .map(|(name, _)| NSString::from_str(name))
            .collect();
        let values: Vec<Retained<MLFeatureValue>> = inputs
            .iter()
            // SAFETY: wrapping an existing array in a feature value.
            .map(|(_, array)| unsafe { MLFeatureValue::featureValueWithMultiArray(array) })
            .collect();
        let key_refs: Vec<&NSString> = keys.iter().map(|key| &**key).collect();
        let value_refs: Vec<&AnyObject> = values.iter().map(|value| value.as_ref()).collect();
        let dictionary = NSDictionary::<NSString, AnyObject>::from_slices(&key_refs, &value_refs);
        // SAFETY: every value is an MLFeatureValue, as the provider requires.
        let provider = unsafe {
            MLDictionaryFeatureProvider::initWithDictionary_error(
                MLDictionaryFeatureProvider::alloc(),
                &dictionary,
            )
        }
        .map_err(|error| anyhow!("building model input: {}", describe(&error)))?;
        // SAFETY: the provider outlives the synchronous prediction call.
        let output = unsafe {
            self.inner.predictionFromFeatures_options_error(
                ProtocolObject::from_ref(&*provider),
                &self.options,
            )
        }
        .map_err(|error| anyhow!("prediction failed: {}", describe(&error)))?;
        Ok(Outputs(output))
    }
}

pub struct Outputs(Retained<ProtocolObject<dyn MLFeatureProvider>>);

impl Outputs {
    pub fn array(&self, name: &str) -> Result<Retained<MLMultiArray>> {
        // SAFETY: reading a named feature from the prediction result.
        unsafe { self.0.featureValueForName(&NSString::from_str(name)) }
            // SAFETY: reading the array of a feature value; `None` if it holds something else.
            .and_then(|value| unsafe { value.multiArrayValue() })
            .with_context(|| format!("model output {name} is missing"))
    }
}

pub fn new_array(
    shape: &[usize],
    data_type: MLMultiArrayDataType,
) -> Result<Retained<MLMultiArray>> {
    let dims = shape
        .iter()
        .map(|&dim| Ok(NSNumber::new_isize(isize::try_from(dim)?)))
        .collect::<Result<Vec<_>>>()?;
    let shape = NSArray::from_retained_slice(&dims);
    // SAFETY: contiguous array of a supported type; callers fill it before use.
    unsafe { MLMultiArray::initWithShape_dataType_error(MLMultiArray::alloc(), &shape, data_type) }
        .map_err(|error| anyhow!("allocating model array: {}", describe(&error)))
}

fn dimensions(numbers: &NSArray<NSNumber>, what: &str) -> Result<Vec<usize>> {
    numbers
        .iter()
        .map(|number| {
            usize::try_from(number.integerValue()).with_context(|| format!("negative {what}"))
        })
        .collect()
}

pub fn shape(array: &MLMultiArray) -> Result<Vec<usize>> {
    // SAFETY: `array` is borrowed, so alive, for the call; the getter has no preconditions and
    // returns a retained array of NSNumbers, as `dimensions` expects.
    dimensions(&*unsafe { array.shape() }, "dimension")
}

fn strides(array: &MLMultiArray) -> Result<Vec<usize>> {
    // SAFETY: `array` is borrowed, so alive, for the call; the getter has no preconditions and
    // returns a retained array of NSNumbers, as `dimensions` expects.
    dimensions(&*unsafe { array.strides() }, "stride")
}

fn offset(index: &[usize], strides: &[usize]) -> usize {
    index
        .iter()
        .zip(strides)
        .map(|(&i, &stride)| i * stride)
        .sum()
}

/// Number of elements a buffer with this layout must hold.
fn required_len(shape: &[usize], strides: &[usize]) -> usize {
    if shape.contains(&0) {
        return 0;
    }
    let last: Vec<usize> = shape.iter().map(|&dim| dim - 1).collect();
    offset(&last, strides) + 1
}

/// An array's elements in storage order, together with the strides that index them.
pub struct Strided<T> {
    pub data: Vec<T>,
    pub shape: Vec<usize>,
    pub strides: Vec<usize>,
}

impl<T: Copy> Strided<T> {
    pub fn get(&self, index: &[usize]) -> T {
        self.data[offset(index, &self.strides)]
    }
}

fn read<T: Copy + Default>(
    array: &MLMultiArray,
    expected: MLMultiArrayDataType,
    label: &str,
) -> Result<Strided<T>> {
    // SAFETY: `array` is borrowed, so alive, for the call, and the getter has no preconditions.
    let data_type = unsafe { array.dataType() };
    ensure!(
        data_type == expected,
        "{label} has unexpected data type {}",
        data_type.0
    );
    let shape = shape(array)?;
    let strides = strides(array)?;
    ensure!(
        shape.len() == strides.len(),
        "{label} has {} dimensions but {} strides",
        shape.len(),
        strides.len()
    );
    let data = RefCell::new(Vec::new());
    let failure = Cell::new(None);
    let handler = StackBlock::new(|bytes: NonNull<c_void>, size: NSInteger| {
        let Ok(size) = usize::try_from(size) else {
            return failure.set(Some(anyhow!("{label} reports a negative buffer size")));
        };
        let base = bytes.as_ptr().cast::<T>();
        *data.borrow_mut() = (0..size / size_of::<T>())
            // SAFETY: CoreML hands out `size` readable bytes for the duration of the handler and the
            // index stays below `size / size_of::<T>()`; CoreML documents no alignment, so, as in
            // `write`, none is assumed.
            .map(|index| unsafe { base.add(index).read_unaligned() })
            .collect();
    });
    // SAFETY: the handler runs synchronously before the call returns.
    unsafe { array.getBytesWithHandler(&handler) };
    if let Some(error) = failure.into_inner() {
        return Err(error);
    }
    let data = data.into_inner();
    ensure!(
        data.len() >= required_len(&shape, &strides),
        "{label} buffer is smaller than its shape"
    );
    Ok(Strided {
        data,
        shape,
        strides,
    })
}

pub fn read_f32(array: &MLMultiArray, label: &str) -> Result<Strided<f32>> {
    read(array, MLMultiArrayDataType::Float32, label)
}

pub fn read_i32(array: &MLMultiArray, label: &str) -> Result<Strided<i32>> {
    read(array, MLMultiArrayDataType::Int32, label)
}

/// Writes `values` in logical (row-major) order into the buffer CoreML hands out; that buffer's
/// strides may differ from the array's `strides`.
fn write<T: Copy>(array: &MLMultiArray, values: &[T]) -> Result<()> {
    let shape = shape(array)?;
    let elements: usize = shape.iter().product();
    ensure!(
        values.len() == elements,
        "writing {} values into an array of {elements}",
        values.len()
    );
    let failure = Cell::new(None);
    let handler = StackBlock::new(
        |bytes: NonNull<c_void>, size: NSInteger, strides: NonNull<NSArray<NSNumber>>| {
            // SAFETY: CoreML passes the buffer's strides for the duration of the handler.
            let strides = match dimensions(unsafe { strides.as_ref() }, "stride") {
                Ok(strides) if strides.len() == shape.len() => strides,
                Ok(_) => {
                    return failure.set(Some(anyhow!("buffer strides do not match the shape")));
                }
                Err(error) => return failure.set(Some(error)),
            };
            let Ok(size) = usize::try_from(size) else {
                return failure.set(Some(anyhow!("buffer reports a negative size")));
            };
            let capacity = size / size_of::<T>();
            if required_len(&shape, &strides) > capacity {
                return failure.set(Some(anyhow!("buffer is smaller than its shape")));
            }
            let base = bytes.as_ptr().cast::<T>();
            let mut index = vec![0usize; shape.len()];
            for &value in values {
                // SAFETY: `index` stays within `shape`, so the offset is below `required_len`,
                // which the buffer's `size` covers.
                unsafe { base.add(offset(&index, &strides)).write_unaligned(value) };
                for axis in (0..shape.len()).rev() {
                    index[axis] += 1;
                    if index[axis] < shape[axis] {
                        break;
                    }
                    index[axis] = 0;
                }
            }
        },
    );
    // SAFETY: the handler runs synchronously before the call returns.
    unsafe { array.getMutableBytesWithHandler(&handler) };
    failure.into_inner().map_or(Ok(()), Err)
}

pub fn write_f32(array: &MLMultiArray, values: &[f32]) -> Result<()> {
    write(array, values)
}

pub fn write_i32(array: &MLMultiArray, values: &[i32]) -> Result<()> {
    write(array, values)
}

pub fn f32_array(shape: &[usize], values: &[f32]) -> Result<Retained<MLMultiArray>> {
    let array = new_array(shape, MLMultiArrayDataType::Float32)?;
    write_f32(&array, values)?;
    Ok(array)
}

pub fn i32_array(shape: &[usize], values: &[i32]) -> Result<Retained<MLMultiArray>> {
    let array = new_array(shape, MLMultiArrayDataType::Int32)?;
    write_i32(&array, values)?;
    Ok(array)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_round_trip_through_a_model_array() {
        let values: Vec<f32> = (0..24).map(|i| i as f32 * 0.5).collect();
        let array = f32_array(&[2, 3, 4], &values).unwrap();
        let read = read_f32(&array, "test").unwrap();
        assert_eq!(read.shape, [2, 3, 4]);
        assert_eq!(read.get(&[1, 2, 3]), 11.5);
        assert_eq!(read.get(&[0, 1, 0]), 2.0);
    }

    #[test]
    fn writing_the_wrong_number_of_values_fails() {
        let array = new_array(&[2, 2], MLMultiArrayDataType::Int32).unwrap();
        assert!(write_i32(&array, &[1, 2, 3]).is_err());
        assert!(write_i32(&array, &[1, 2, 3, 4]).is_ok());
    }
}
