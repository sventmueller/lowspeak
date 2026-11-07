<p><img src="../../readme-icon.png" alt="Icono de la app Lowspeak" width="168" height="168"></p>

# Lowspeak

**Dictado para el Mac que escribe donde está tu cursor: mantén pulsada fn, habla y suelta.**

Lowspeak vive en la barra de menús. Mantén pulsada la tecla fn (globo terráqueo) y habla; al soltarla, tus palabras aparecen en la app en la que estés escribiendo, como si las hubieras tecleado tú. Transcribe en línea con OpenAI o Groq, o completamente en tu Mac con Whisper o NVIDIA Parakeet, que convierte 40 segundos de voz en texto en aproximadamente un tercio de segundo. Si la red o el servicio en línea fallan, un modelo local instalado toma el relevo, así que no se pierde ningún dictado. Enséñale los nombres y términos que usas, deja que un modelo de lenguaje elimine las muletillas si quieres, y bloquea una grabación con Shift para dictar textos más largos sin manos. Está escrito en Rust sobre controles genuinos de AppKit, habla 35 idiomas y solo envía tu voz al proveedor que hayas elegido.

[![Descargar Lowspeak para macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Descargar Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: firmado y notarizado por Apple, para macOS 14 o posterior en Apple Silicon e Intel.

**¿Has encontrado un error?** [Abre una incidencia](https://github.com/sventmueller/lowspeak/issues/new/choose) o elige **Ayuda › Informar de un problema…** en el menú de Lowspeak: el formulario se abre con tu versión ya rellenada. Si Lowspeak te resulta útil, una [estrella en GitHub](https://github.com/sventmueller/lowspeak) ayuda a que otros lo encuentren.

[![Rust, edición 2024](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 o posterior](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universal: Apple Silicon e Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit mediante objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp sobre Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet en el Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![En línea con OpenAI o Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 idiomas de interfaz](https://img.shields.io/badge/languages-35-4C9A2A)
![Firmado con Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Licencia: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#licencia)

[Bahasa Indonesia](../id/README.md) ·
[Bahasa Melayu](../ms/README.md) ·
[Català](../ca/README.md) ·
[Čeština](../cs/README.md) ·
[Dansk](../da/README.md) ·
[Deutsch](../de/README.md) ·
[English](../../../README.md) ·
**Español** ·
[Français](../fr/README.md) ·
[Hrvatski](../hr/README.md) ·
[Italiano](../it/README.md) ·
[Magyar](../hu/README.md) ·
[Nederlands](../nl/README.md) ·
[Norsk](../no/README.md) ·
[Polski](../pl/README.md) ·
[Português (Brasil)](../pt_BR/README.md) ·
[Português (Portugal)](../pt_PT/README.md) ·
[Română](../ro/README.md) ·
[Slovenčina](../sk/README.md) ·
[Slovenščina](../sl/README.md) ·
[Suomi](../fi/README.md) ·
[Svenska](../sv/README.md) ·
[Tiếng Việt](../vi/README.md) ·
[Türkçe](../tr/README.md) ·
[Ελληνικά](../el/README.md) ·
[Русский](../ru/README.md) ·
[Українська](../uk/README.md) ·
[עברית](../he/README.md)‎ ·
[العربية](../ar/README.md)‎ ·
[हिन्दी](../hi/README.md) ·
[ไทย](../th/README.md) ·
[한국어](../ko/README.md) ·
[日本語](../ja/README.md) ·
[简体中文](../zh-Hans/README.md) ·
[繁體中文](../zh-Hant/README.md)

## Contenido

- [Resumen de funciones](#resumen-de-funciones)
- [Requisitos](#requisitos)
- [Instalación](#instalación)
- [Primeros pasos](#primeros-pasos)
  - [Permisos](#permisos)
  - [Elige cómo transcribir](#elige-cómo-transcribir)
  - [El menú](#el-menú)
  - [Referencia rápida](#referencia-rápida)
- [Dictar](#dictar)
  - [Mantén pulsada para hablar](#mantén-pulsada-para-hablar)
  - [Bloquea con Shift o con una doble pulsación](#bloquea-con-shift-o-con-una-doble-pulsación)
  - [Adónde va el texto](#adónde-va-el-texto)
  - [Dictados recientes](#dictados-recientes)
- [Transcripción](#transcripción)
  - [Qué modelo te conviene](#qué-modelo-te-conviene)
  - [En línea: OpenAI y Groq](#en-línea-openai-y-groq)
  - [En tu Mac: Whisper y Parakeet](#en-tu-mac-whisper-y-parakeet)
  - [Cuando falla el servicio en línea](#cuando-falla-el-servicio-en-línea)
  - [Cómo se prepara la grabación](#cómo-se-prepara-la-grabación)
  - [Idiomas por modelo](#idiomas-por-modelo)
  - [Mac con Intel](#mac-con-intel)
- [Vocabulario](#vocabulario)
- [Posprocesado](#posprocesado)
- [Micrófono](#micrófono)
- [Claves de API](#claves-de-api)
- [Notificaciones](#notificaciones)
- [Apariencia y accesibilidad](#apariencia-y-accesibilidad)
- [Idiomas](#idiomas)
- [Teclas y gestos](#teclas-y-gestos)
- [Privacidad y seguridad](#privacidad-y-seguridad)
- [Velocidad](#velocidad)
- [Limitaciones conocidas](#limitaciones-conocidas)
- [Solución de problemas](#solución-de-problemas)
- [Para desarrolladores](#para-desarrolladores)
  - [Componentes de terceros](#componentes-de-terceros)
- [Licencia](#licencia)

## Resumen de funciones

- **Mantén pulsada fn, habla y suelta.** El texto se escribe en la app que está en primer plano, allí donde esté el cursor: correo, chat, editor, terminal o navegador. Sin ventanas a las que cambiar ni botones que pulsar.
- **Bloquéala para textos más largos.** Pulsa Shift mientras mantienes pulsada fn, o pulsa fn dos veces, y la grabación sigue sin que tengas que mantener ninguna tecla pulsada, hasta cinco minutos. Una pulsación de fn o Shift la termina.
- **El modelo adecuado para cada tarea.** Groq para ir rápido sin gastar la memoria de tu Mac, GPT Transcribe de OpenAI para el mejor resultado, Whisper o NVIDIA Parakeet en tu Mac para trabajar sin conexión y mantener tu voz en privado. Cambia de uno a otro con un solo clic.
- **Ningún dictado perdido.** Si la red o el servicio en línea fallan, un modelo local instalado transcribe la misma grabación en tu Mac.
- **Tus palabras, bien escritas.** Un vocabulario de nombres y términos técnicos guía a OpenAI, Groq, Whisper y al posprocesado.
- **Texto limpio si lo quieres.** El posprocesado opcional elimina las muletillas, corrige la gramática y, cuando te corriges a ti mismo, conserva solo la corrección; o, además, da formato al texto según el tipo de app en la que dictas.
- **Sigue hablando.** Empieza el siguiente dictado mientras el anterior aún se está transcribiendo; los textos llegan en el orden en que los dijiste.
- **Tu portapapeles sigue siendo tuyo.** Lowspeak pega a través del portapapeles y, un momento después, vuelve a dejar en él lo que había. Los diez últimos dictados se quedan en el menú para copiarlos de nuevo.
- **Nativo hasta la médula.** Un ítem real de la barra de menús con un menú de AppKit, alertas y notificaciones estándar. Sin Electron, sin ventana, sin icono en el Dock.
- **Privado.** Sin cuenta ni telemetría. El audio solo va al proveedor que hayas elegido, o a ninguna parte en el modo local.
- **En tu idioma.** El dictado funciona en decenas de idiomas, y el menú, los diálogos y las notificaciones están disponibles en 35.

## Requisitos

- macOS 14 Sonoma o posterior. Lowspeak se desarrolla y se prueba en macOS 27.
- Un Mac con Apple Silicon o con procesador Intel. La transcripción local está pensada para Apple Silicon, donde Whisper se ejecuta en la GPU y Parakeet en el Neural Engine; en los Mac con Intel, ambos se ejecutan en el procesador, mucho más despacio; consulta [Mac con Intel](#mac-con-intel).
- Unos 17 MB para la app, más entre 75 MB y 3 GB por cada modelo local que descargues.
- Para la transcripción en línea, una clave de API de [OpenAI](https://platform.openai.com/api-keys) o de [Groq](https://console.groq.com/keys); ambos cobran por lo que transcribes. La transcripción local no necesita ninguna cuenta.

## Instalación

**Desde la imagen de disco.** Descarga [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), ábrela y arrastra Lowspeak a la carpeta Aplicaciones. La imagen está firmada con un Developer ID y notarizada por Apple, así que se abre sin avisos.

**Con Homebrew.** El cask está en un tap propio:

```sh
brew install --cask sventmueller/tap/lowspeak
```

`brew uninstall --zap --cask lowspeak` elimina además los ajustes, los modelos, los registros y las cachés de Lowspeak; tus claves de API se quedan en el llavero de inicio de sesión.

**Actualizaciones.** Lowspeak, instalado desde la imagen de disco o con Homebrew, se mantiene al día por sí solo con [Sparkle](https://sparkle-project.org). En el segundo arranque pregunta si puede buscar actualizaciones por su cuenta, una vez al día. **Buscar actualizaciones…** en el menú busca al instante; **Buscar actualizaciones automáticamente** y **Descargar e instalar actualizaciones automáticamente**, justo debajo, cambian después tu elección. Si una búsqueda encuentra una versión nueva mientras trabajas en otra app, aparece un punto en el anillo de la barra de menús, y **Buscar actualizaciones…** trae la actualización al primer plano. Lowspeak solo instala una actualización si lleva la firma del desarrollador; tus ajustes, claves y modelos se conservan. `brew upgrade --cask lowspeak` también actualiza una copia instalada con Homebrew. La versión 1.0.0 no tiene actualizador: instala encima una versión más reciente una sola vez, desde la [página de versiones](https://github.com/sventmueller/lowspeak/releases). Una compilación propia tampoco tiene actualizador.

**Desde el código fuente.** Consulta [Compilación y despliegue](../../../docs/development.md#build--deploy) para ver los requisitos previos y después:

```sh
make deploy
```

Esto compila Lowspeak, sustituye la copia de tu carpeta Aplicaciones y lo abre.

**Abrir al iniciar sesión**, en el menú, inicia Lowspeak junto con tu Mac mediante los ítems de inicio del propio macOS. Solo se ejecuta una copia a la vez; una segunda se cierra de inmediato.

## Primeros pasos

### Permisos

En el primer arranque, Lowspeak pide tres permisos:

- **Accesibilidad** (Ajustes del Sistema › Privacidad y seguridad › Accesibilidad; en macOS 27 la lista se llama Control de dispositivos y acceso a datos). Lowspeak lo necesita para detectar la tecla fn y para pegar con ⌘V. macOS ofrece abrir la lista; activa el interruptor de Lowspeak. Los mensajes de Lowspeak nombran la lista tal como la muestra tu macOS.
- **Micrófono.** Se pide una vez; sin él, Lowspeak no graba nada.
- **Notificaciones.** Lowspeak las usa para problemas que de otro modo no verías, como un micrófono sin señal o una transcripción fallida.

### Elige cómo transcribir

Configura al menos una forma; puedes cambiarla en cualquier momento desde el menú.

- **En línea:** elige **Configurar clave de API…** y pega una clave de OpenAI (empieza por `sk-`) o de Groq (`gsk_`). Lowspeak reconoce el proveedor por la clave. Después elige un modelo en **OpenAI** o **Groq**.
- **En tu Mac:** elige **Gestionar modelos** › **Descargar Parakeet Ultra** o un modelo Whisper, y luego selecciónalo en **Local**.

[Qué modelo te conviene](#qué-modelo-te-conviene) te ayuda a elegir. Después, mantén pulsada fn, di algo y suelta.

### El menú

Haz clic en el anillo de la barra de menús:

| Ítem | Qué hace |
|---|---|
| **Dictados recientes** | los diez últimos textos; un clic vuelve a copiar uno al portapapeles |
| **Local** | los modelos de tu Mac, agrupados en Whisper y Parakeet; los que no están descargados aparecen atenuados |
| **OpenAI**, **Groq** | los modelos en línea; sin clave aparecen atenuados y el menú ofrece **Configurar clave de API…** |
| **Idioma** | el idioma en el que dictas, o **Detección automática**; el menú lo sigue |
| **Posprocesado** | **Desactivado**, **Limpieza** o **Formato**; consulta [Posprocesado](#posprocesado) |
| **Vocabulario…** | nombres y términos que deben escribirse tal cual; el menú muestra cuántos has introducido |
| **Micrófono** | **Ajuste por omisión del sistema** o un micrófono fijo, y **Reproducir sonido al estar listo** |
| **Gestionar modelos** | descargar y eliminar modelos locales, con su progreso |
| **Configurar clave de API…** / **Actualizar clave de API…** | añadir, sustituir o eliminar claves |
| **Abrir al iniciar sesión** | iniciar Lowspeak junto con tu Mac; la marca indica que está activado |
| **Buscar actualizaciones…** | buscar una versión nueva ahora; los dos ítems de debajo deciden si Lowspeak busca una vez al día e instala las actualizaciones sin preguntar. Solo en la app de la imagen de disco |
| **Ayuda** | **Referencia rápida**, **Informar de un problema…**, **Sugerir una función…**, **Lowspeak en GitHub** |
| **Acerca de Lowspeak** | versión, copyright, página del proyecto y licencia |
| **Salir de Lowspeak** | ⌘Q con el menú abierto |

La marca en **Local**, **OpenAI** y **Groq** señala el modelo en uso. El icono muestra lo que está haciendo Lowspeak:

| Icono | Significado |
|---|---|
| Anillo con un punto | Listo |
| Círculo relleno | Grabando mientras se mantiene pulsada fn |
| Cuadrado redondeado | Grabación bloqueada; la siguiente pulsación de fn o Shift la termina |
| Anillo parpadeante | Transcribiendo |
| Anillo con un punto arriba a la derecha | Hay una versión nueva esperando; consulta **Buscar actualizaciones…** |
| ↓ 42 % | Descargando un modelo |

Lowspeak no tiene ventana de ajustes: todo está en este menú, y recuerda cada elección.

### Referencia rápida

**Ayuda › Referencia rápida** muestra los gestos de fn en una pequeña ventana, en tu idioma, con un botón que abre los ajustes de Teclado. **Informar de un problema…** abre un formulario en GitHub con tus versiones de Lowspeak y macOS ya rellenadas; **Sugerir una función…**, una discusión para proponer ideas; **Lowspeak en GitHub**, la página del proyecto.

## Dictar

### Mantén pulsada para hablar

Mantén pulsada fn, habla y suelta. El icono de grabación aparece en cuanto tu micrófono entrega sonido; empieza a hablar entonces, porque lo que digas antes no se graba. Al soltar fn termina la grabación, y el texto llega un momento después.

Una pulsación breve de fn no hace nada, así que fn conserva sus otros usos. Una grabación que queda vacía o dura menos de 0,3 segundos se descarta.

### Bloquea con Shift o con una doble pulsación

Para textos más largos, bloquea la grabación para que siga sin que tengas que mantener ninguna tecla pulsada. Hay tres formas:

- **Shift mientras mantienes pulsada fn.** Empieza con fn como siempre y pulsa Shift durante la grabación: se bloquea y puedes soltar fn. No se pierde nada de lo que hayas dicho; la grabación simplemente continúa.
- **Shift y luego fn.** Mantén pulsada la tecla Shift y pulsa fn: la grabación empieza bloqueada.
- **fn dos veces.** Pulsa fn brevemente (menos de 0,3 segundos) y vuelve a pulsarla antes de que pasen 0,4 segundos. La grabación cuenta desde la primera pulsación.

El icono se convierte en un cuadrado redondeado. Pulsa **fn** o **Shift** una vez para detenerla; el texto llega como siempre. Si tu tecla 🌐 está configurada para abrir Emojis y símbolos, detén la grabación con Shift: una pulsación breve de fn abriría el visor.

Una grabación se detiene sola al cabo de dos minutos mientras se mantiene pulsada fn, y al cabo de cinco minutos cuando está bloqueada, y después se transcribe como siempre.

### Adónde va el texto

Lowspeak pega el texto en la app que estaba en primer plano cuando dejaste de hablar, en la posición del cursor, a través del portapapeles y ⌘V. Guarda todo lo que había antes en el portapapeles y lo restaura medio segundo después de que la app haya leído el texto, o a los diez segundos si no llega a leerlo, salvo que otra app haya cambiado el portapapeles entretanto. El texto se marca como transitorio, así que las apps de historial del portapapeles que respetan esa marca no lo registran.

Puedes empezar el siguiente dictado mientras el anterior aún se está transcribiendo. Los textos se pegan en el orden en que los dijiste, y nunca mientras estás grabando o mantienes pulsada la tecla Shift, de modo que un pegado no puede caer en mitad de tu siguiente frase.

Sin el permiso de Accesibilidad, o si no se puede enviar el ⌘V simulado, el texto se queda en el portapapeles y una notificación te lo indica; pégalo tú con ⌘V.

### Dictados recientes

**Dictados recientes**, en el menú, muestra los diez últimos textos, del más reciente al más antiguo, cada uno acortado a una línea; al pasar el puntero por encima se ve el texto completo. Un clic lo copia al portapapeles, por si un texto acabó en el lugar equivocado o lo necesitas dos veces. La lista solo existe en memoria y queda vacía al salir de Lowspeak.

## Transcripción

### Qué modelo te conviene

| Quieres | Elige | Por qué |
|---|---|---|
| **Velocidad sin cargar tu Mac** | **Groq › Whisper Large V3 Turbo** | la opción en línea más rápida; no se descarga nada, y tu Mac conserva su memoria y su espacio en disco |
| **El mejor resultado** | **OpenAI › GPT Transcribe** (por omisión) | el más preciso de los modelos que ofrece Lowspeak; recibe tu vocabulario como palabras clave |
| **Sin conexión y en privado** | **Local › Parakeet Ultra** para idiomas europeos, **Local › Whisper Large v3 Turbo** para todos los demás | funciona sin red, y tu voz nunca sale de tu Mac; Parakeet es el más rápido de todos |
| **Sin cuenta ni costes** | cualquier modelo **Local** | se ejecuta en tu Mac; no se factura nada |
| **Fiabilidad y velocidad a la vez** | un modelo en línea, más Parakeet Ultra o un modelo Whisper instalado | el modelo local entra en acción cuando falla el servicio en línea |
| **Un Mac con Intel** | **Groq** u **OpenAI** | allí los modelos locales se ejecutan en el procesador; consulta [Mac con Intel](#mac-con-intel) |

Los modelos locales ocupan en disco lo mismo que su descarga y, mientras están seleccionados, aproximadamente lo mismo en memoria; Lowspeak mantiene el modelo cargado para que el dictado empiece sin demora. En un Mac con 8 GB de memoria, es preferible Parakeet Ultra o un modelo en línea a los modelos grandes de Whisper.

### En línea: OpenAI y Groq

| Modelo | Proveedor | Notas |
|---|---|---|
| **GPT Transcribe** (por omisión) | OpenAI | el más preciso; recibe tu vocabulario como palabras clave |
| **Whisper** | OpenAI | el modelo Whisper original |
| **Whisper Large V3** | Groq | muy rápido |
| **Whisper Large V3 Turbo** | Groq | el más rápido en línea |

La grabación se comprime en AAC antes de subirla, unos 6 KB por segundo de voz, así que el dictado funciona también con conexiones lentas. Mientras mantienes pulsada fn, Lowspeak ya abre la conexión con el proveedor, de modo que la subida empieza en cuanto sueltas la tecla.

### En tu Mac: Whisper y Parakeet

| Modelo | Tamaño | Se ejecuta en | Idiomas |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 idiomas europeos |
| **Parakeet v2** | ~600 MB | Neural Engine | inglés |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | unos 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | unos 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | unos 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | unos 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | unos 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | unos 99 |

**Gestionar modelos** los descarga y los elimina; durante una descarga, la barra de menús muestra su progreso, y al salir de Lowspeak se cancela. El primer uso de un modelo lo prepara para tu Mac: unos 10 segundos para Parakeet Ultra y hasta 30 segundos para Whisper; después se carga en una fracción de segundo. Si se elimina el modelo que elegiste, Lowspeak cambia a otro de los instalados.

**Parakeet** es la opción más rápida: transcribe 40 segundos de voz en unos 0,3 segundos. Lowspeak lo ejecuta con los mismos modelos y la misma decodificación que [FluidAudio](https://github.com/FluidInference/FluidAudio) y los guarda en la carpeta de FluidAudio (`~/Library/Application Support/FluidAudio/Models`), así que las apps basadas en FluidAudio los comparten. Las grabaciones de más de 15 segundos se transcriben en ventanas superpuestas que se unen sin perder palabras en las juntas. **Whisper** entiende más idiomas y lee tu vocabulario; los modelos grandes son los más precisos, y los pequeños, mucho más flojos fuera del inglés. Los modelos Whisper se guardan en `~/Library/Application Support/Lowspeak/Models`.

### Cuando falla el servicio en línea

Si una transcripción en línea falla, porque la red no funciona, el proveedor tiene un problema o la clave es incorrecta, un modelo local instalado transcribe la misma grabación: el elegido en **Local** o, si no, Parakeet Ultra, luego Whisper Large v3 Turbo, Large v3 y los modelos Whisper más pequeños, y Parakeet v2 en último lugar. Una notificación te avisa una vez; solo vuelve a aparecer después de que un dictado en línea haya funcionado entretanto.

Con un modelo local de reserva, Lowspeak da 20 segundos a la solicitud en línea; sin él, lo intenta tres veces durante un minuto como máximo cada vez. En el modo en línea, Lowspeak prepara una vez al arrancar el modelo Parakeet de reserva, para que la primera vez que haga falta no haya que esperarlo.

### Cómo se prepara la grabación

Lowspeak graba a la frecuencia de muestreo propia de tu micrófono y, cuando te detienes, la convierte a los 16 kHz que esperan los modelos. Antes de la transcripción elimina el retumbo grave por debajo de 80 Hz, recorta el silencio al principio y al final e iguala el volumen, de modo que las voces bajas y las altas salen igual. Una grabación sin voz ni siquiera se envía. En el modo local, Parakeet recibe primero la grabación sin tocar, y la preparada solo si esa no da ningún texto.

### Idiomas por modelo

El idioma elegido en **Idioma** se pasa a los modelos en línea y a Whisper. Con **Detección automática**, el modelo averigua el idioma por sí mismo.

| Modelo | Idiomas en los que puedes dictar |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, en tu Mac) | unos 99, entre ellos los 35 del menú |
| Parakeet Ultra | alemán, búlgaro, checo, croata, danés, eslovaco, esloveno, español, estonio, finés, francés, griego, húngaro, inglés, italiano, letón, lituano, maltés, neerlandés, polaco, portugués, rumano, ruso, sueco, ucraniano |
| Parakeet v2 | inglés |

Parakeet detecta el idioma por sí mismo e ignora la elección del menú. **Chino (tradicional)** indica a GPT Transcribe que escriba caracteres tradicionales; Whisper escribe el chino en cualquiera de las dos escrituras, y el portugués sin tener en cuenta la variante.

### Mac con Intel

Lowspeak es una app universal y también funciona en los Mac con Intel con macOS 14. Todo lo que va en línea funciona igual. Allí, los modelos locales no tienen el hardware para el que están hechos: Whisper se ejecuta en el procesador en lugar de en la GPU, y Parakeet en el procesador en lugar de en el Neural Engine. Ambos funcionan, pero tardan muchas veces más que en Apple Silicon, y los modelos grandes de Whisper, demasiado para dictar. En un Mac con Intel, transcribe en línea y deja Whisper Small o Parakeet Ultra como [reserva](#cuando-falla-el-servicio-en-línea).

## Vocabulario

**Vocabulario…** recoge los nombres y términos que Lowspeak debe escribir exactamente, separados por comas: nombres de productos, compañeros de trabajo, términos técnicos. Hasta 100 términos de hasta 60 caracteres cada uno; `<` y `>` se eliminan.

- **GPT Transcribe** los recibe como palabras clave.
- **Whisper** (OpenAI, Groq y en tu Mac) los lee como pista antes de la grabación, hasta 500 caracteres.
- **El posprocesado** recibe la indicación de escribirlos exactamente como están escritos.
- **Parakeet** no los usa.

## Posprocesado

Después de la transcripción, un modelo de lenguaje puede reelaborar el texto. Elige en **Posprocesado**:

- **Desactivado** (por omisión): el texto tal como se transcribió.
- **Limpieza**: elimina muletillas como «eh», «o sea» o «básicamente», corrige la gramática y la puntuación y, cuando te corriges a ti mismo, conserva solo la corrección. No añade nada ni cambia el significado.
- **Formato**: como Limpieza, y además adapta el texto a la app en la que dictas: términos con estilo de código en los editores de código, líneas de comandos en los terminales, un tono informal en las apps de chat y uno profesional en el correo.

El posprocesado usa Groq (`openai/gpt-oss-120b`) si tienes una clave de Groq y, si no, OpenAI (`gpt-4o-mini`), también cuando transcribes en local. Si hay un idioma elegido en **Idioma**, lo mantiene. Si tarda más de ocho segundos, falla o devuelve una respuesta cortada o mucho más larga que lo que dijiste, Lowspeak pega el texto sin procesar. Si el proveedor rechaza la solicitud o no hay ninguna clave configurada, una notificación te lo indica una vez.

## Micrófono

En **Micrófono**, elige un micrófono fijo o **Ajuste por omisión del sistema**. Lowspeak recuerda el dispositivo; mientras no está conectado, el menú lo muestra como **(desconectado)** y Lowspeak graba con el micrófono por omisión del sistema.

El micrófono solo se enciende mientras dictas. Ponerlo a punto lleva unos 0,07 segundos con el micrófono integrado, aproximadamente medio segundo con un Studio Display y de uno a dos segundos con unos AirPods, que primero cambian a su modo de auriculares con micrófono. El icono de grabación aparece en cuanto llega el sonido; **Reproducir sonido al estar listo** añade un breve sonido en ese momento. Está desactivado por omisión, porque en algunos altavoces el propio sonido llega medio segundo tarde.

Si un micrófono solo entrega silencio durante cinco segundos, como ocurre con los AirPods cuando otro dispositivo tiene ocupado su micrófono, Lowspeak te avisa.

## Claves de API

**Configurar clave de API…** pide una clave y reconoce el proveedor por su comienzo: `sk-` para OpenAI, `gsk_` para Groq. Una clave con otro formato se rechaza con una explicación. Introduce una clave por proveedor; con claves de ambos, están disponibles todos los modelos en línea. **Actualizar clave de API…** sustituye una clave o, con **Eliminar todas las claves**, las borra.

Las claves se guardan en tu llavero de inicio de sesión bajo `com.lowspeak.app`. El archivo de claves de una versión anterior (`~/Library/Application Support/Lowspeak/.api-key`) se traslada allí en el siguiente arranque y se elimina en cuanto todas las claves pueden volver a leerse del llavero.

## Notificaciones

Lowspeak trabaja en segundo plano, así que informa de los problemas mediante notificaciones:

| Notificación | Qué ha pasado |
|---|---|
| **Error de grabación** | no hay acceso al micrófono, no hay ningún micrófono conectado o el micrófono no ha podido iniciarse |
| **Micrófono sin señal** | cinco segundos de silencio; consulta [Micrófono](#micrófono) |
| **Se necesita permiso de accesibilidad** | el texto está en el portapapeles; concede el permiso de Accesibilidad para pegar automáticamente |
| **Error al pegar** | el texto está en el portapapeles; pégalo con ⌘V |
| **Error de transcripción** | con el motivo: no hay clave, clave rechazada, límite de solicitudes, error del servidor o de red, o ningún modelo local |
| **Transcrito en este Mac** | el servicio en línea falló y un modelo local tomó el relevo |
| **Error de posprocesado** | el proveedor rechaza el posprocesado o no hay ninguna clave configurada; el texto se pegó sin él |
| **Sin clave de API** | elegiste un modelo en línea sin clave para su proveedor |
| **Error de descarga**, **Error al eliminar**, **No se puede eliminar** | la gestión de un modelo local no funcionó, u otra operación seguía en curso |
| **Error del ítem de inicio**, **Error de clave de API** | macOS o el llavero rechazó el cambio |

Las notificaciones nunca contienen el texto que has dictado. Un servicio en línea que falla y un proveedor de posprocesado que rechaza las solicitudes se notifican una sola vez, no tras cada dictado, hasta que vuelven a funcionar.

## Apariencia y accesibilidad

- **El icono de la barra de menús** es una imagen de plantilla que se adapta a una barra de menús clara, oscura o tintada.
- **VoiceOver** lee el icono como «Lowspeak», y el menú, las alertas y las notificaciones son controles estándar de macOS.
- **No hace falta mantener ninguna tecla pulsada.** Bloquea una grabación con Shift o con una doble pulsación de fn, y la siguiente pulsación la termina.
- **La referencia rápida** es una ventana sencilla con etiquetas y un botón, accesible desde el teclado; Esc la cierra.

Las carencias conocidas se recogen en [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver no anuncia cuándo empieza una grabación, y Lowspeak todavía no se ha probado con VoiceOver ni con Control por botón.

## Idiomas

El menú, los diálogos y las notificaciones están traducidos a 35 idiomas:

alemán, árabe, catalán, checo, chino (simplificado), chino (tradicional), coreano, croata, danés, eslovaco, esloveno, español, finés, francés, griego, hebreo, hindi, húngaro, indonesio, inglés, italiano, japonés, malayo, neerlandés, noruego, polaco, portugués (Brasil), portugués (Portugal), rumano, ruso, sueco, tailandés, turco, ucraniano y vietnamita.

El idioma que elijas en **Idioma** es también el idioma del menú. Con **Detección automática**, el menú sigue el orden de idiomas de **Ajustes del Sistema › General › Idioma y región**, y usa el inglés si no hay traducción. Los términos de los menús siguen las traducciones de Apple, así que Lowspeak se lee como el resto de tu Mac. Salvo el inglés, las traducciones son automáticas y todavía no las han revisado hablantes nativos; las correcciones son bienvenidas como [incidencias](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Teclas y gestos

| Acción | Teclas |
|---|---|
| Dictar mientras se mantiene pulsada | mantener pulsada fn |
| Bloquear una grabación en curso | Shift mientras mantienes pulsada fn |
| Iniciar una grabación bloqueada | Shift+fn, o fn dos veces rápido |
| Terminar una grabación bloqueada | fn o Shift |
| Salir de Lowspeak (con el menú abierto) | ⌘Q |
| Cerrar la referencia rápida | Esc |

Lowspeak impide que las pulsaciones de fn que usa lleguen a otras apps; Shift pasa como siempre.

## Privacidad y seguridad

Lowspeak oye lo que dices mientras mantienes pulsada fn, puede escribir en cualquier app y guarda tus claves de API, así que hace con todo ello lo mínimo posible.

- **Tu voz va a un solo sitio.** En el modo en línea, la grabación, comprimida, va al proveedor que hayas elegido, OpenAI o Groq, junto con tu vocabulario. En el modo local, nada sale de tu Mac. Si una solicitud en línea falla, un modelo local transcribe la misma grabación en tu Mac.
- **El posprocesado**, si lo activas, envía el texto transcrito y tu vocabulario a Groq o a OpenAI. En el modo Formato añade el tipo de app en la que dictas (editor de código, terminal, chat o correo); el nombre de la app no se envía.
- **Sin telemetría ni cuenta.** Aparte de la transcripción y el posprocesado, Lowspeak solo se conecta a internet para abrir la conexión con tu proveedor mientras mantienes pulsada fn, en el modo en línea o con el posprocesado activado, para descargar un modelo cuando lo pides, desde Hugging Face por HTTPS, en una versión fija y verificando la suma de comprobación de cada archivo, y para buscar una versión nueva en GitHub, una vez al día si lo permites. Esa solicitud lleva la versión de Lowspeak y nada más. **Ayuda › Informar de un problema…** abre un formulario de GitHub en tu navegador; su dirección lleva la versión de Lowspeak y la de macOS, nada de lo que hayas dictado.
- **Las claves de API** se quedan en tu llavero de inicio de sesión. Solo se envían a su propio proveedor y se eliminan de los mensajes de error antes de que se escriba nada en el registro.
- **El texto dictado no se guarda.** Los diez últimos dictados permanecen en memoria para **Dictados recientes** y desaparecen al salir de Lowspeak. Los archivos de registro anotan longitudes y tiempos, nunca el texto.
- **El portapapeles** se guarda antes de pegar y se restaura en cuanto la app ha leído el texto, salvo que otra app lo haya cambiado entretanto. El dictado se marca como transitorio, así que las apps de historial del portapapeles que respetan esa marca lo omiten.
- **Firma de código** con el Hardened Runtime; la app solo pide acceso al micrófono, a Accesibilidad, a las notificaciones y a la red. Una actualización debe llevar la firma del desarrollador; si no, Sparkle la rechaza.

Para informar de una vulnerabilidad, sigue la [política de seguridad](../../../SECURITY.md).

## Velocidad

Medido en Apple Silicon:

- **Parakeet Ultra** transcribe 40 segundos de voz en unos 0,3 segundos.
- **En línea:** la primera solicitud tras una pausa tarda unos 0,4 segundos en Groq, porque Lowspeak abre la conexión mientras aún estás hablando; las siguientes la reutilizan, mediante HTTP/3 cuando el proveedor lo ofrece.
- **Tamaño de la subida:** unos 6 KB por segundo de voz, en AAC a 48 kbit/s.
- **Puesta a punto del micrófono:** unos 0,07 segundos con el integrado, medio segundo con un Studio Display y de uno a dos segundos con AirPods.

## Limitaciones conocidas

- **Los modelos locales en los Mac con Intel** se ejecutan en el procesador y son mucho más lentos; allí es mejor transcribir en línea.
- **Lo que digas antes de que el micrófono esté listo se pierde.** Espera al icono de grabación, sobre todo con AirPods.
- **Otros pueden quedarse con la tecla fn.** Si el ajuste de la tecla 🌐 de macOS, Warp o el Dictado de macOS reclaman fn antes, Lowspeak no la ve; consulta [Solución de problemas](#solución-de-problemas).
- **Parakeet** ignora el vocabulario y la elección de idioma, y solo conoce idiomas europeos.
- **Escritura y variante:** solo a GPT Transcribe se le puede indicar que escriba chino tradicional; a ningún modelo se le puede indicar que escriba portugués europeo en lugar de brasileño.
- **Límites de tiempo:** dos minutos mientras se mantiene pulsada fn, cinco minutos con la grabación bloqueada.
- **Modo de micrófono:** el centro de control ofrece Aislamiento de voz y Sonido de fondo para Lowspeak y los muestra como activos, pero Lowspeak graba sin procesamiento de voz, así que la elección no tiene ningún efecto.
- **Dictados recientes** solo existe en memoria y desaparece al salir.
- **Por diseño:** sin ventana de ajustes, sin icono en el Dock y fuera del App Store.

## Solución de problemas

**No pasa nada al mantener pulsada fn.** Comprueba tres cosas: Lowspeak tiene permiso en **Ajustes del Sistema › Privacidad y seguridad › Accesibilidad** (en macOS 27: **Control de dispositivos y acceso a datos**); **Ajustes del Sistema › Teclado › Acción al pulsar la tecla 🌐** está en **No hacer nada**; y ninguna otra app reclama fn. Warp lo hace por omisión; desactívalo con `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` y reinicia Warp. El Dictado de macOS también puede estar configurado para empezar con fn.

**Tras una actualización, Lowspeak ya no reacciona a fn.** macOS vincula el permiso de Accesibilidad a la firma de la app. Si la firma ha cambiado, por ejemplo al pasar de una compilación propia a la app descargada, quita Lowspeak de la lista de Accesibilidad (en macOS 27: Control de dispositivos y acceso a datos) y vuelve a añadirlo.

**Faltan las primeras palabras.** Empieza a hablar en cuanto aparezca el icono de grabación. Con AirPods, el micrófono necesita de uno a dos segundos; un micrófono integrado o USB elegido en **Micrófono** está listo casi al instante.

**«Micrófono sin señal».** El micrófono no ha entregado sonido durante cinco segundos. Con AirPods, puede que otro dispositivo esté usando su micrófono; cambia al micrófono integrado o vuelve a conectarlos. Con la tapa del MacBook cerrada, el micrófono integrado también queda silenciado.

**Los dictados salen vacíos.** Puede que el volumen de entrada esté demasiado bajo: apps como Teams, Zoom o Meet lo cambian durante las llamadas, y macOS lo guarda por dispositivo. Súbelo en **Ajustes del Sistema › Sonido › Entrada**.

**El texto acabó en el lugar equivocado.** **Dictados recientes**, en el menú, lo vuelve a copiar al portapapeles.

**«Sin clave de API».** Elige **Configurar clave de API…** en el menú o cambia a un modelo local.

**La primera transcripción local tarda mucho.** Cada modelo se prepara para tu Mac la primera vez que se usa; a partir de entonces se carga en una fracción de segundo.

**Lowspeak se cierra justo después de abrirlo.** Ya hay otra copia de Lowspeak en ejecución; ciérrala primero.

**No aparece ninguna notificación.** Permítelas en **Ajustes del Sistema › Notificaciones › Lowspeak**.

**Otra cosa.** Los registros de `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) describen lo que ocurrió, sin el texto que dictaste. Adjunta las líneas relevantes a un [informe de problema](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Para desarrolladores

La compilación, las pruebas, la estructura del código y las reglas que sigue se describen en [docs/development.md](../../../docs/development.md); las contribuciones, en [CONTRIBUTING.md](../../../CONTRIBUTING.md).

### Componentes de terceros

| Componente | Versión | Para qué se usa | Licencia |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) mediante [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | Whisper local en la GPU (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | decodificación de Parakeet y unión de grabaciones largas, traducidas a Rust en `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) y sus crates de frameworks | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation y otros frameworks del sistema | MIT; Zlib, Apache-2.0 o MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | captura del micrófono mediante CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | remuestreo a 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) con rustls | 3.5 | descarga de modelos | MIT o Apache-2.0; rustls y ring bajo ISC y Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | sumas de comprobación de las descargas de modelos | MIT o Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | claves de API en el llavero | MIT o Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, errores, el vocabulario de Parakeet | MIT o Apache-2.0; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | actualizaciones de la app de la imagen de disco | MIT, con partes bajo licencias BSD y zlib |

Los textos de licencia de todo lo que contiene la app se incluyen con ella como `Contents/Resources/Acknowledgements.txt`.

Los modelos no forman parte de Lowspeak; los descarga cuando se lo pides:

| Modelo | Origen | Licencia |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (el Whisper de OpenAI en formato ggml) | MIT |
| Parakeet v2, Parakeet Ultra | conversiones a CoreML de [FluidInference](https://huggingface.co/FluidInference) del Parakeet TDT de NVIDIA | CC-BY-4.0 |

## Licencia

Lowspeak se distribuye bajo cualquiera de las dos licencias siguientes:

- Licencia Apache, versión 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- Licencia MIT ([LICENSE-MIT](../../../LICENSE-MIT))

a tu elección.

Salvo que declares expresamente lo contrario, toda contribución enviada intencionadamente para su inclusión en Lowspeak por tu parte, tal como se define en la licencia Apache-2.0, tendrá doble licencia como arriba, sin términos ni condiciones adicionales.
