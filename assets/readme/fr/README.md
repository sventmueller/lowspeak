<p><img src="../../readme-icon.png" alt="Icône de l’app Lowspeak" width="168" height="168"></p>

# Lowspeak

**La dictée pour Mac qui écrit là où se trouve votre curseur : maintenez fn, parlez, relâchez.**

Lowspeak se loge dans la barre des menus. Maintenez la touche fn (Globe) enfoncée et parlez ; dès que vous la relâchez, vos mots apparaissent dans l’app où vous êtes en train d’écrire, comme si vous les aviez tapés vous-même. Il transcrit en ligne avec OpenAI ou Groq, ou entièrement sur votre Mac avec Whisper ou NVIDIA Parakeet, qui transforme 40 secondes de parole en texte en environ un tiers de seconde. Si le réseau ou le service en ligne fait défaut, un modèle local installé prend le relais : aucune dictée n’est perdue. Apprenez-lui les noms et les termes que vous employez, laissez un modèle de langage supprimer les mots parasites si vous le souhaitez, et verrouillez un enregistrement avec Shift pour dicter des textes plus longs en mains libres. Il est écrit en Rust avec de véritables contrôles AppKit, parle 35 langues et n’envoie votre voix qu’au fournisseur que vous avez choisi.

[![Télécharger Lowspeak pour macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Télécharger Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)** : signé et notarisé par Apple, pour macOS 14 ou ultérieur, sur Apple Silicon et Intel.

**Vous avez trouvé un bogue ?** [Ouvrez une issue](https://github.com/sventmueller/lowspeak/issues/new/choose), ou choisissez **Aide › Signaler un problème…** dans le menu de Lowspeak : le formulaire s’ouvre avec votre version déjà renseignée. Si Lowspeak vous est utile, une [étoile sur GitHub](https://github.com/sventmueller/lowspeak) aide d’autres personnes à le trouver.

[![Rust, édition 2024](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 ou ultérieur](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universel : Apple Silicon et Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit via objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp sur Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet sur le Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![En ligne avec OpenAI ou Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 langues d’interface](https://img.shields.io/badge/languages-35-4C9A2A)
![Signé avec Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Licence : MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#licence)

[Bahasa Indonesia](../id/README.md) ·
[Bahasa Melayu](../ms/README.md) ·
[Català](../ca/README.md) ·
[Čeština](../cs/README.md) ·
[Dansk](../da/README.md) ·
[Deutsch](../de/README.md) ·
[English](../../../README.md) ·
[Español](../es/README.md) ·
**Français** ·
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

## Sommaire

- [Aperçu des fonctionnalités](#aperçu-des-fonctionnalités)
- [Configuration requise](#configuration-requise)
- [Installation](#installation)
- [Premiers pas](#premiers-pas)
  - [Autorisations](#autorisations)
  - [Choisir le mode de transcription](#choisir-le-mode-de-transcription)
  - [Le menu](#le-menu)
  - [Aide-mémoire](#aide-mémoire)
- [Dicter](#dicter)
  - [Maintenir pour parler](#maintenir-pour-parler)
  - [Verrouiller avec Shift ou une double pression](#verrouiller-avec-shift-ou-une-double-pression)
  - [Où va le texte](#où-va-le-texte)
  - [Dictées récentes](#dictées-récentes)
- [Transcription](#transcription)
  - [Quel modèle vous convient](#quel-modèle-vous-convient)
  - [En ligne : OpenAI et Groq](#en-ligne-openai-et-groq)
  - [Sur votre Mac : Whisper et Parakeet](#sur-votre-mac-whisper-et-parakeet)
  - [Quand le service en ligne échoue](#quand-le-service-en-ligne-échoue)
  - [Comment l’enregistrement est préparé](#comment-lenregistrement-est-préparé)
  - [Langues par modèle](#langues-par-modèle)
  - [Mac Intel](#mac-intel)
- [Vocabulaire](#vocabulaire)
- [Post-traitement](#post-traitement)
- [Micro](#micro)
- [Clés API](#clés-api)
- [Notifications](#notifications)
- [Apparence et accessibilité](#apparence-et-accessibilité)
- [Langues](#langues)
- [Touches et gestes](#touches-et-gestes)
- [Confidentialité et sécurité](#confidentialité-et-sécurité)
- [Vitesse](#vitesse)
- [Limites connues](#limites-connues)
- [Dépannage](#dépannage)
- [Pour les développeurs](#pour-les-développeurs)
  - [Composants tiers](#composants-tiers)
- [Licence](#licence)

## Aperçu des fonctionnalités

- **Maintenez fn, parlez, relâchez.** Le texte est saisi dans l’app au premier plan, là où se trouve le curseur : courriel, messagerie, éditeur, terminal ou navigateur. Aucune fenêtre vers laquelle basculer, aucun bouton sur lequel cliquer.
- **Verrouillez-le pour les textes plus longs.** Appuyez sur Shift tout en maintenant fn, ou appuyez deux fois sur fn, et l’enregistrement continue sans qu’aucune touche ne soit maintenue, jusqu’à cinq minutes. Une pression sur fn ou Shift y met fin.
- **Le bon modèle pour chaque usage.** Groq pour la vitesse sans solliciter la mémoire de votre Mac, GPT Transcribe d’OpenAI pour le meilleur résultat, Whisper ou NVIDIA Parakeet sur votre Mac pour travailler hors ligne et garder votre voix privée. Passez de l’un à l’autre en un clic.
- **Aucune dictée perdue.** Si le réseau ou le service en ligne fait défaut, un modèle local installé transcrit le même enregistrement sur votre Mac.
- **Vos mots, bien orthographiés.** Un vocabulaire de noms et de termes techniques guide OpenAI, Groq, Whisper et le post-traitement.
- **Un texte propre si vous le souhaitez.** Le post-traitement facultatif supprime les mots parasites, corrige la grammaire et ne garde que votre correction lorsque vous vous reprenez, ou met aussi le texte en forme selon le type d’app dans laquelle vous dictez.
- **Continuez à parler.** Lancez la dictée suivante pendant que la précédente est encore en cours de transcription ; les textes arrivent dans l’ordre où vous les avez prononcés.
- **Votre presse-papiers reste le vôtre.** Lowspeak colle via le presse-papiers et y remet un instant plus tard ce qu’il contenait. Les dix dernières dictées restent dans le menu pour être copiées à nouveau.
- **Natif jusqu’au bout des ongles.** Un véritable élément de la barre des menus avec un menu AppKit, des alertes et des notifications standard. Pas d’Electron, pas de fenêtre, pas d’icône dans le Dock.
- **Confidentiel.** Pas de compte, pas de télémétrie. L’audio ne va qu’au fournisseur que vous avez choisi, ou nulle part en mode local.
- **Dans votre langue.** La dictée fonctionne dans des dizaines de langues, et le menu, les dialogues et les notifications existent en 35 langues.

## Configuration requise

- macOS 14 Sonoma ou ultérieur. Lowspeak est développé et testé sous macOS 27.
- Un Mac avec Apple Silicon ou un processeur Intel. La transcription locale est conçue pour Apple Silicon, où Whisper s’exécute sur le GPU et Parakeet sur le Neural Engine ; sur les Mac Intel, les deux s’exécutent sur le processeur, beaucoup plus lentement, voir [Mac Intel](#mac-intel).
- Environ 17 Mo pour l’app, plus 75 Mo à 3 Go pour chaque modèle local que vous téléchargez.
- Pour la transcription en ligne, une clé API d’[OpenAI](https://platform.openai.com/api-keys) ou de [Groq](https://console.groq.com/keys) ; les deux facturent ce que vous transcrivez. La transcription locale ne nécessite aucun compte.

## Installation

**Depuis l’image disque.** Téléchargez [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), ouvrez-la et faites glisser Lowspeak dans le dossier Applications. L’image est signée avec un Developer ID et notarisée par Apple : elle s’ouvre donc sans avertissement.

**Mises à jour.** Lowspeak, installé depuis l’image disque, se met à jour tout seul grâce à [Sparkle](https://sparkle-project.org). Au deuxième lancement, il demande s’il peut rechercher des mises à jour de lui-même, une fois par jour. **Rechercher les mises à jour…** dans le menu lance la recherche tout de suite ; **Rechercher automatiquement les mises à jour** et **Télécharger et installer automatiquement les mises à jour**, juste en dessous, permettent de changer ce choix plus tard. Si une recherche trouve une nouvelle version pendant que vous travaillez dans une autre app, un point apparaît sur l’anneau dans la barre des menus, et **Rechercher les mises à jour…** fait passer la mise à jour au premier plan. Lowspeak n’installe une mise à jour que si elle porte la signature du développeur ; vos réglages, vos clés et vos modèles sont conservés. La version 1.0.0 n’a pas de mise à jour automatique : installez une seule fois une version plus récente par-dessus, depuis la [page des versions](https://github.com/sventmueller/lowspeak/releases). Une version compilée par vos soins n’a pas non plus de mise à jour automatique.

**Depuis les sources.** Consultez [Build & Deploy](../../../docs/development.md#build--deploy) pour les prérequis, puis :

```sh
make deploy
```

Cette commande compile Lowspeak, remplace la copie dans votre dossier Applications et la lance.

**Ouvrir avec la session** dans le menu lance Lowspeak en même temps que votre Mac, via les éléments d’ouverture de macOS. Une seule copie s’exécute à la fois ; une deuxième se ferme aussitôt.

## Premiers pas

### Autorisations

Au premier lancement, Lowspeak demande trois autorisations :

- **Accessibilité** (Réglages Système › Confidentialité et sécurité › Accessibilité ; sous macOS 27, la liste s’appelle Contrôle de l’appareil et accès aux données). Lowspeak en a besoin pour détecter la touche fn et pour coller avec ⌘V. macOS propose d’ouvrir la liste ; activez l’interrupteur de Lowspeak. Les messages de Lowspeak nomment la liste comme votre macOS l’affiche.
- **Micro.** Demandée une seule fois ; sans elle, Lowspeak n’enregistre rien.
- **Notifications.** Lowspeak s’en sert pour les problèmes que vous ne remarqueriez pas autrement, comme un micro silencieux ou une transcription qui a échoué.

### Choisir le mode de transcription

Configurez au moins un mode ; vous pouvez en changer à tout moment dans le menu.

- **En ligne :** choisissez **Définir la clé API…** et collez une clé d’OpenAI (elle commence par `sk-`) ou de Groq (`gsk_`). Lowspeak reconnaît le fournisseur à la clé. Choisissez ensuite un modèle sous **OpenAI** ou **Groq**.
- **Sur votre Mac :** choisissez **Gérer les modèles** › **Télécharger Parakeet Ultra** ou un modèle Whisper, puis sélectionnez-le sous **Local**.

[Quel modèle vous convient](#quel-modèle-vous-convient) vous aide à choisir. Ensuite, maintenez fn, dites quelque chose et relâchez.

### Le menu

Cliquez sur l’anneau dans la barre des menus :

| Élément | Fonction |
|---|---|
| **Dictées récentes** | les dix derniers textes ; un clic en copie un à nouveau dans le presse-papiers |
| **Local** | les modèles sur votre Mac, regroupés en Whisper et Parakeet ; ceux qui ne sont pas téléchargés sont grisés |
| **OpenAI**, **Groq** | les modèles en ligne ; sans clé, ils sont grisés et le menu propose **Définir la clé API…** |
| **Langue** | la langue dans laquelle vous dictez, ou **Détection automatique** ; le menu suit cette langue |
| **Post-traitement** | **Désactivé**, **Nettoyage** ou **Mise en forme**, voir [Post-traitement](#post-traitement) |
| **Vocabulaire…** | les noms et les termes à écrire exactement ; le menu indique combien vous en avez saisi |
| **Micro** | **Réglage par défaut du système** ou un micro fixe, et **Signal sonore une fois prêt** |
| **Gérer les modèles** | télécharger et supprimer les modèles locaux, avec la progression |
| **Définir la clé API…** / **Modifier la clé API…** | ajouter, remplacer ou supprimer des clés |
| **Ouvrir avec la session** | lancer Lowspeak en même temps que votre Mac ; la coche indique que l’option est activée |
| **Rechercher les mises à jour…** | rechercher une nouvelle version maintenant ; les deux éléments situés en dessous déterminent si Lowspeak effectue une recherche une fois par jour et installe les mises à jour sans demander. Uniquement dans l’app installée depuis l’image disque |
| **Aide** | **Aide-mémoire**, **Signaler un problème…**, **Suggérer une fonction…**, **Lowspeak sur GitHub** |
| **À propos de Lowspeak** | version, copyright, page du projet et licence |
| **Quitter Lowspeak** | ⌘Q lorsque le menu est ouvert |

La coche sous **Local**, **OpenAI** et **Groq** indique le modèle utilisé. L’icône montre ce que fait Lowspeak :

| Icône | Signification |
|---|---|
| Anneau avec un point | Prêt |
| Cercle plein | Enregistrement pendant que fn est maintenue |
| Carré arrondi | Enregistrement verrouillé ; la prochaine pression sur fn ou Shift y met fin |
| Anneau clignotant | Transcription en cours |
| Anneau avec un point en haut à droite | Une nouvelle version est en attente, voir **Rechercher les mises à jour…** |
| ↓ 42 % | Téléchargement d’un modèle |

Lowspeak n’a pas de fenêtre de réglages : tout se trouve dans ce menu, et il mémorise chacun de vos choix.

### Aide-mémoire

**Aide › Aide-mémoire** affiche les gestes de la touche fn dans une petite fenêtre, dans votre langue, avec un bouton qui ouvre les réglages Clavier. **Signaler un problème…** ouvre un formulaire sur GitHub avec vos versions de Lowspeak et de macOS déjà renseignées, **Suggérer une fonction…** une discussion pour les idées, **Lowspeak sur GitHub** la page du projet.

## Dicter

### Maintenir pour parler

Maintenez fn, parlez, relâchez. L’icône d’enregistrement apparaît dès que votre micro fournit du son ; commencez à parler à ce moment-là, car ce que vous dites avant n’est pas enregistré. Relâcher fn met fin à l’enregistrement, et le texte suit un instant plus tard.

Une brève pression sur fn ne fait rien : fn conserve ainsi ses autres usages. Un enregistrement qui reste vide ou dure moins de 0,3 seconde est ignoré.

### Verrouiller avec Shift ou une double pression

Pour les textes plus longs, verrouillez l’enregistrement afin qu’il continue sans qu’aucune touche ne soit maintenue. Il y a trois façons de le faire :

- **Shift en maintenant fn.** Commencez avec fn comme d’habitude et appuyez sur Shift pendant l’enregistrement : il se verrouille, et vous pouvez relâcher fn. Rien de ce que vous avez dit n’est perdu, l’enregistrement continue simplement.
- **Shift, puis fn.** Maintenez Shift et appuyez sur fn : l’enregistrement démarre verrouillé.
- **Deux fois fn.** Appuyez brièvement sur fn (moins de 0,3 seconde) et appuyez de nouveau dans les 0,4 seconde qui suivent. L’enregistrement commence dès la première pression.

L’icône devient un carré arrondi. Appuyez une fois sur **fn** ou **Shift** pour arrêter ; le texte suit comme d’habitude. Si votre touche 🌐 est réglée pour ouvrir Emoji et symboles, arrêtez avec Shift : une brève pression sur fn ouvrirait le visualiseur.

Un enregistrement s’arrête de lui-même au bout de deux minutes lorsque fn est maintenue, et au bout de cinq minutes lorsqu’il est verrouillé ; il est alors transcrit comme d’habitude.

### Où va le texte

Lowspeak colle le texte dans l’app qui était au premier plan lorsque vous avez cessé de parler, à l’emplacement du curseur, via le presse-papiers et ⌘V. Il sauvegarde tout ce que contenait le presse-papiers auparavant et le restaure une demi-seconde après que l’app a lu le texte, ou au bout de dix secondes si elle ne le lit jamais, sauf si une autre app a modifié le presse-papiers entre-temps. Le texte est marqué comme transitoire, si bien que les apps d’historique du presse-papiers qui respectent ce marqueur ne l’enregistrent pas.

Vous pouvez lancer la dictée suivante pendant que la précédente est encore en cours de transcription. Les textes sont collés dans l’ordre où vous les avez prononcés, et jamais pendant que vous enregistrez ou maintenez Shift : un collage ne peut donc pas atterrir au milieu de votre phrase suivante.

Sans l’autorisation Accessibilité, ou si le ⌘V simulé ne peut pas être envoyé, le texte reste dans le presse-papiers et une notification vous le signale ; collez-le vous-même avec ⌘V.

### Dictées récentes

**Dictées récentes** dans le menu liste les dix derniers textes, du plus récent au plus ancien, chacun réduit à une ligne ; le survol affiche le texte entier. Un clic le copie dans le presse-papiers, pour le cas où un texte a atterri au mauvais endroit ou si vous en avez besoin deux fois. La liste n’existe qu’en mémoire et se vide quand vous quittez Lowspeak.

## Transcription

### Quel modèle vous convient

| Vous voulez | Choisissez | Pourquoi |
|---|---|---|
| **De la vitesse sans solliciter votre Mac** | **Groq › Whisper Large V3 Turbo** | l’option en ligne la plus rapide ; rien n’est téléchargé, et votre Mac garde sa mémoire et son espace disque |
| **Le meilleur résultat** | **OpenAI › GPT Transcribe** (par défaut) | le plus précis des modèles proposés par Lowspeak ; reçoit votre vocabulaire sous forme de mots-clés |
| **Hors ligne et en toute confidentialité** | **Local › Parakeet Ultra** pour les langues européennes, **Local › Whisper Large v3 Turbo** pour toutes les autres | fonctionne sans réseau, et votre voix ne quitte jamais votre Mac ; Parakeet est le plus rapide de tous |
| **Pas de compte, pas de frais** | n’importe quel modèle **Local** | s’exécute sur votre Mac ; rien n’est facturé |
| **À la fois fiabilité et vitesse** | un modèle en ligne, plus Parakeet Ultra ou un modèle Whisper installé | le modèle local prend le relais quand le service en ligne fait défaut |
| **Un Mac Intel** | **Groq** ou **OpenAI** | les modèles locaux y tournent sur le processeur, voir [Mac Intel](#mac-intel) |

Les modèles locaux occupent sur le disque l’équivalent de leur taille de téléchargement et, tant qu’ils sont sélectionnés, à peu près autant de mémoire ; Lowspeak garde le modèle chargé pour que la dictée démarre sans délai. Sur un Mac doté de 8 Go de mémoire, préférez Parakeet Ultra ou un modèle en ligne aux grands modèles Whisper.

### En ligne : OpenAI et Groq

| Modèle | Fournisseur | Remarques |
|---|---|---|
| **GPT Transcribe** (par défaut) | OpenAI | le plus précis ; reçoit votre vocabulaire sous forme de mots-clés |
| **Whisper** | OpenAI | le modèle Whisper d’origine |
| **Whisper Large V3** | Groq | très rapide |
| **Whisper Large V3 Turbo** | Groq | le plus rapide en ligne |

L’enregistrement est compressé en AAC avant l’envoi, environ 6 Ko par seconde de parole, si bien que la dictée fonctionne aussi sur des connexions lentes. Pendant que vous maintenez fn, Lowspeak ouvre déjà la connexion au fournisseur, de sorte que l’envoi commence dès que vous relâchez la touche.

### Sur votre Mac : Whisper et Parakeet

| Modèle | Taille | S’exécute sur | Langues |
|---|---|---|---|
| **Parakeet Ultra** | ~630 Mo | Neural Engine | 25 langues européennes |
| **Parakeet v2** | ~600 Mo | Neural Engine | anglais |
| **Whisper Large v3 Turbo** | ~1,6 Go | GPU (Metal) | environ 99 |
| **Whisper Large v3** | ~3 Go | GPU (Metal) | environ 99 |
| **Whisper Medium** | ~1,5 Go | GPU (Metal) | environ 99 |
| **Whisper Small** | ~470 Mo | GPU (Metal) | environ 99 |
| **Whisper Base** | ~150 Mo | GPU (Metal) | environ 99 |
| **Whisper Tiny** | ~75 Mo | GPU (Metal) | environ 99 |

**Gérer les modèles** les télécharge et les supprime ; pendant un téléchargement, la barre des menus en affiche la progression, et quitter Lowspeak l’annule. La toute première utilisation d’un modèle le prépare pour votre Mac : environ 10 secondes pour Parakeet Ultra, jusqu’à 30 secondes pour Whisper ; ensuite, il se charge en une fraction de seconde. Si le modèle que vous avez choisi est supprimé, Lowspeak passe à un autre modèle installé.

**Parakeet** est le choix le plus rapide : il transcrit 40 secondes de parole en environ 0,3 seconde. Lowspeak l’exécute avec les mêmes modèles et le même décodage que [FluidAudio](https://github.com/FluidInference/FluidAudio) et les stocke dans le dossier de FluidAudio (`~/Library/Application Support/FluidAudio/Models`), si bien que les apps basées sur FluidAudio les partagent. Les enregistrements de plus de 15 secondes sont transcrits par fenêtres qui se chevauchent et qui sont raccordées sans perdre de mots aux jointures. **Whisper** comprend davantage de langues et lit votre vocabulaire ; les grands modèles sont les plus précis, les petits bien plus faibles en dehors de l’anglais. Les modèles Whisper sont stockés dans `~/Library/Application Support/Lowspeak/Models`.

### Quand le service en ligne échoue

Si une transcription en ligne échoue, parce que le réseau est coupé, que le fournisseur rencontre un problème ou que la clé est erronée, un modèle local installé transcrit le même enregistrement : celui choisi sous **Local**, sinon Parakeet Ultra, puis Whisper Large v3 Turbo, Large v3 et les modèles Whisper plus petits, et Parakeet v2 en dernier. Une notification vous en informe une fois ; elle ne revient qu’après qu’une dictée en ligne a réussi entre-temps.

Avec un modèle local de secours, Lowspeak accorde 20 secondes à la requête en ligne ; sans modèle de secours, il fait trois tentatives d’une minute au plus chacune. En mode en ligne, Lowspeak prépare une fois au lancement le modèle Parakeet de secours, pour que le premier recours au secours n’ait pas à l’attendre.

### Comment l’enregistrement est préparé

Lowspeak enregistre à la fréquence propre de votre micro et, une fois l’enregistrement arrêté, le convertit aux 16 kHz qu’attendent les modèles. Avant la transcription, il supprime le grondement grave sous 80 Hz, coupe le silence au début et à la fin et égalise le volume, de sorte que les voix faibles et les voix fortes ressortent de la même façon. Un enregistrement sans parole n’est pas envoyé du tout. En mode local, Parakeet reçoit d’abord l’enregistrement brut, et la version préparée seulement si celui-ci ne donne aucun texte.

### Langues par modèle

La langue choisie sous **Langue** est transmise aux modèles en ligne et à Whisper. Avec **Détection automatique**, le modèle détermine la langue lui-même.

| Modèle | Langues que vous pouvez dicter |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, sur votre Mac) | environ 99, dont les 35 du menu |
| Parakeet Ultra | allemand, anglais, bulgare, croate, danois, espagnol, estonien, finnois, français, grec, hongrois, italien, letton, lituanien, maltais, néerlandais, polonais, portugais, roumain, russe, slovaque, slovène, suédois, tchèque, ukrainien |
| Parakeet v2 | anglais |

Parakeet détecte la langue lui-même et ignore le choix fait dans le menu. **Chinois (traditionnel)** indique à GPT Transcribe d’écrire en caractères traditionnels ; Whisper écrit le chinois dans l’une ou l’autre écriture, et le portugais sans tenir compte de la variante.

### Mac Intel

Lowspeak est une app universelle et fonctionne aussi sur les Mac Intel sous macOS 14. Tout ce qui passe par Internet fonctionne de la même manière. Les modèles locaux n’y disposent pas du matériel pour lequel ils sont conçus : Whisper s’exécute sur le processeur au lieu du GPU, et Parakeet sur le processeur au lieu du Neural Engine. Les deux fonctionnent, mais prennent plusieurs fois plus de temps que sur Apple Silicon, et les grands modèles Whisper bien trop pour la dictée. Sur un Mac Intel, transcrivez en ligne et gardez Whisper Small ou Parakeet Ultra comme [modèle de secours](#quand-le-service-en-ligne-échoue).

## Vocabulaire

**Vocabulaire…** accepte les noms et les termes que Lowspeak doit écrire exactement, séparés par des virgules : noms de produits, collègues, termes techniques. Jusqu’à 100 termes de 60 caractères au plus chacun ; `<` et `>` sont supprimés.

- **GPT Transcribe** les reçoit sous forme de mots-clés.
- **Whisper** (OpenAI, Groq et sur votre Mac) les lit comme indication avant l’enregistrement, jusqu’à 500 caractères.
- **Le post-traitement** reçoit la consigne de les écrire exactement tels quels.
- **Parakeet** ne les utilise pas.

## Post-traitement

Après la transcription, un modèle de langage peut retravailler le texte. Choisissez sous **Post-traitement** :

- **Désactivé** (par défaut) — le texte tel que transcrit.
- **Nettoyage** — supprime les mots parasites comme « euh », « genre » ou « en fait », corrige la grammaire et la ponctuation, et ne garde que la correction lorsque vous vous reprenez. Il n’ajoute rien et ne change pas le sens.
- **Mise en forme** — comme Nettoyage, et adapte en plus le texte à l’app dans laquelle vous dictez : termes au format code dans les éditeurs de code, lignes de commande dans les terminaux, ton décontracté dans les apps de messagerie, ton professionnel dans les courriels.

Le post-traitement utilise Groq (`openai/gpt-oss-120b`) si vous avez une clé Groq, sinon OpenAI (`gpt-4o-mini`), y compris lorsque vous transcrivez en local. Si une langue est choisie sous **Langue**, il conserve cette langue. S’il prend plus de huit secondes, échoue ou renvoie une réponse tronquée ou bien plus longue que ce que vous avez dit, Lowspeak colle le texte non traité. Si le fournisseur refuse la requête ou si aucune clé n’est définie, une notification vous le signale une fois.

## Micro

Sous **Micro**, choisissez un micro fixe ou **Réglage par défaut du système**. Lowspeak mémorise l’appareil ; tant qu’il n’est pas connecté, le menu l’affiche avec la mention **(non connecté)** et Lowspeak enregistre avec le micro par défaut du système.

Le micro n’est activé que pendant que vous dictez. Sa mise en route prend environ 0,07 seconde avec le micro intégré, environ une demi-seconde avec un Studio Display, et une à deux secondes avec des AirPods, qui passent d’abord en mode casque. L’icône d’enregistrement apparaît dès que le son arrive ; **Signal sonore une fois prêt** ajoute un bref son à ce moment-là. Cette option est désactivée par défaut, car sur certains haut-parleurs le son lui-même arrive avec une demi-seconde de retard.

Si un micro ne fournit que du silence pendant cinq secondes, comme les AirPods lorsqu’un autre appareil occupe leur micro, Lowspeak vous en avertit.

## Clés API

**Définir la clé API…** demande une clé et reconnaît le fournisseur à son début : `sk-` pour OpenAI, `gsk_` pour Groq. Une clé dans un autre format est refusée avec une explication. Saisissez une clé par fournisseur ; avec des clés pour les deux, tous les modèles en ligne sont disponibles. **Modifier la clé API…** remplace une clé ou, avec **Supprimer toutes les clés**, les efface.

Les clés sont stockées dans votre trousseau de session sous `com.lowspeak.app`. Le fichier de clés d’une version antérieure (`~/Library/Application Support/Lowspeak/.api-key`) y est déplacé au lancement suivant et supprimé dès que chaque clé peut être relue depuis le trousseau.

## Notifications

Lowspeak fonctionne en arrière-plan ; il signale donc les problèmes par des notifications :

| Notification | Ce qui s’est passé |
|---|---|
| **Échec de l’enregistrement** | pas d’accès au micro, aucun micro connecté, ou le micro n’a pas pu démarrer |
| **Micro silencieux** | cinq secondes de silence, voir [Micro](#micro) |
| **Autorisation d’accessibilité requise** | le texte est dans le presse-papiers ; accordez l’autorisation Accessibilité pour coller automatiquement |
| **Échec du collage** | le texte est dans le presse-papiers ; collez-le avec ⌘V |
| **Échec de la transcription** | avec la raison : pas de clé, clé refusée, limite de requêtes, erreur du serveur ou du réseau, ou aucun modèle local |
| **Transcrit sur ce Mac** | le service en ligne a échoué et un modèle local a pris le relais |
| **Échec du post-traitement** | le fournisseur refuse le post-traitement ou aucune clé n’est définie ; le texte a été collé sans |
| **Aucune clé API** | vous avez choisi un modèle en ligne sans clé pour son fournisseur |
| **Échec du téléchargement**, **Échec de la suppression**, **Suppression impossible** | la gestion d’un modèle local n’a pas fonctionné, ou une autre opération était encore en cours |
| **Erreur d’ouverture avec la session**, **Erreur de clé API** | macOS ou le trousseau a refusé la modification |

Les notifications ne contiennent jamais le texte que vous avez dicté. Un service en ligne défaillant et un fournisseur de post-traitement qui refuse les requêtes sont signalés une seule fois, et non après chaque dictée, jusqu’à ce qu’ils fonctionnent de nouveau.

## Apparence et accessibilité

- **L’icône de la barre des menus** est une image modèle qui s’adapte à une barre des menus claire, sombre ou teintée.
- **VoiceOver** lit l’icône comme « Lowspeak », et le menu, les alertes et les notifications sont des contrôles macOS standard.
- **Aucune touche ne doit rester enfoncée.** Verrouillez un enregistrement avec Shift ou une double pression sur fn, et la pression suivante y met fin.
- **L’Aide-mémoire** est une simple fenêtre avec des libellés et un bouton, accessible au clavier ; Esc la ferme.

Les lacunes connues sont répertoriées dans [ACCESSIBILITY.md](../../../ACCESSIBILITY.md) : VoiceOver n’annonce pas le début d’un enregistrement, et Lowspeak n’a pas encore été testé avec VoiceOver ni avec Contrôle de sélection.

## Langues

Le menu, les dialogues et les notifications sont traduits en 35 langues :

allemand, anglais, arabe, catalan, chinois (simplifié), chinois (traditionnel), coréen, croate, danois, espagnol, finnois, français, grec, hébreu, hindi, hongrois, indonésien, italien, japonais, malais, néerlandais, norvégien, polonais, portugais (Brésil), portugais (Portugal), roumain, russe, slovaque, slovène, suédois, tchèque, thaï, turc, ukrainien et vietnamien.

La langue que vous choisissez sous **Langue** est aussi celle du menu. Avec **Détection automatique**, le menu suit l’ordre des langues défini dans **Réglages Système › Général › Langue et région**, et utilise l’anglais lorsqu’aucune traduction n’existe. Les termes des menus reprennent les traductions d’Apple, de sorte que Lowspeak se lit comme le reste de votre Mac. En dehors de l’anglais, les traductions sont automatiques et n’ont pas encore été relues par des locuteurs natifs ; les corrections sont les bienvenues sous forme d’[issues](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Touches et gestes

| Action | Touches |
|---|---|
| Dicter tant que la touche est maintenue | maintenir fn |
| Verrouiller un enregistrement en cours | Shift en maintenant fn |
| Démarrer un enregistrement verrouillé | Shift+fn, ou deux fois fn rapidement |
| Terminer un enregistrement verrouillé | fn ou Shift |
| Quitter Lowspeak (menu ouvert) | ⌘Q |
| Fermer l’Aide-mémoire | Esc |

Lowspeak empêche les pressions sur fn qu’il utilise d’atteindre les autres apps ; Shift passe normalement.

## Confidentialité et sécurité

Lowspeak entend ce que vous dites tant que fn est maintenue, peut écrire dans n’importe quelle app et conserve vos clés API ; il en fait donc le moins possible.

- **Votre voix ne va qu’à un seul endroit.** En mode en ligne, l’enregistrement, compressé, est envoyé au fournisseur que vous avez choisi, OpenAI ou Groq, avec votre vocabulaire. En mode local, rien ne quitte votre Mac. Si une requête en ligne échoue, un modèle local transcrit le même enregistrement sur votre Mac.
- **Le post-traitement**, si vous l’activez, envoie le texte transcrit et votre vocabulaire à Groq ou à OpenAI. En mode Mise en forme, il y ajoute le type d’app dans laquelle vous dictez (éditeur de code, terminal, messagerie ou courriel) ; le nom de l’app n’est pas envoyé.
- **Pas de télémétrie, pas de compte.** En dehors de la transcription et du post-traitement, Lowspeak ne se connecte à Internet que pour ouvrir la connexion à votre fournisseur pendant que fn est maintenue, en mode en ligne ou avec le post-traitement activé, pour télécharger un modèle quand vous le demandez, depuis Hugging Face via HTTPS, dans une version fixe et en vérifiant la somme de contrôle de chaque fichier, et pour rechercher une nouvelle version sur GitHub, une fois par jour si vous l’autorisez. Cette requête contient la version de Lowspeak et rien d’autre. **Aide › Signaler un problème…** ouvre un formulaire sur GitHub dans votre navigateur ; son adresse contient la version de Lowspeak et celle de macOS, rien de ce que vous avez dicté.
- **Les clés API** restent dans votre trousseau de session. Elles ne sont envoyées qu’à leur propre fournisseur et sont retirées des messages d’erreur avant que quoi que ce soit ne soit écrit dans le journal.
- **Le texte dicté n’est pas stocké.** Les dix dernières dictées restent en mémoire pour **Dictées récentes** et disparaissent quand vous quittez Lowspeak. Les fichiers journaux enregistrent des longueurs et des durées, jamais le texte.
- **Le presse-papiers** est sauvegardé avant le collage et restauré une fois que l’app a lu le texte, sauf si une autre app l’a modifié entre-temps. La dictée est marquée comme transitoire, si bien que les apps d’historique du presse-papiers qui respectent ce marqueur l’ignorent.
- **Signature du code** avec le Hardened Runtime ; l’app ne demande que l’accès au micro, à l’accessibilité, aux notifications et au réseau. Une mise à jour doit porter la signature du développeur, sinon Sparkle la refuse.

Pour signaler une vulnérabilité, suivez la [politique de sécurité](../../../SECURITY.md).

## Vitesse

Mesures effectuées sur Apple Silicon :

- **Parakeet Ultra** transcrit 40 secondes de parole en environ 0,3 seconde.
- **En ligne :** la première requête après une pause prend environ 0,4 seconde chez Groq, car Lowspeak ouvre la connexion pendant que vous parlez encore ; les requêtes suivantes la réutilisent, via HTTP/3 lorsque le fournisseur le propose.
- **Taille de l’envoi :** environ 6 Ko par seconde de parole, en AAC à 48 kbit/s.
- **Mise en route du micro :** environ 0,07 seconde pour le micro intégré, une demi-seconde sur un Studio Display, une à deux secondes avec des AirPods.

## Limites connues

- **Les modèles locaux sur les Mac Intel** s’exécutent sur le processeur et sont beaucoup plus lents ; la transcription en ligne y est le meilleur choix.
- **Ce que vous dites avant que le micro soit prêt est perdu.** Attendez l’icône d’enregistrement, surtout avec des AirPods.
- **La touche fn peut être accaparée par d’autres.** Lorsque le réglage de la touche 🌐 de macOS, Warp ou la Dictée de macOS s’emparent de fn en premier, Lowspeak ne la voit pas ; voir [Dépannage](#dépannage).
- **Parakeet** ignore le vocabulaire et le choix de la langue, et ne connaît que des langues européennes.
- **Écriture et variante :** seul GPT Transcribe peut recevoir la consigne d’écrire en chinois traditionnel ; aucun modèle ne peut recevoir celle d’écrire en portugais européen plutôt que brésilien.
- **Limites de durée :** deux minutes lorsque fn est maintenue, cinq minutes lorsque l’enregistrement est verrouillé.
- **Mode du micro :** le Centre de contrôle propose Isolement de la voix et Large spectre pour Lowspeak et les indique comme actifs, mais Lowspeak enregistre sans traitement de la voix : ce choix n’a donc aucun effet.
- **Dictées récentes** n’existe qu’en mémoire et disparaît quand vous quittez Lowspeak.
- **Par choix :** pas de fenêtre de réglages, pas d’icône dans le Dock, pas de présence dans l’App Store.

## Dépannage

**Rien ne se passe quand je maintiens fn.** Vérifiez trois choses : Lowspeak est autorisé dans **Réglages Système › Confidentialité et sécurité › Accessibilité** (sous macOS 27 : **Contrôle de l’appareil et accès aux données**) ; **Réglages Système › Clavier › Appuyer sur la touche 🌐 pour** est réglé sur **Ne rien faire** ; et aucune autre app ne s’approprie fn. Warp le fait par défaut ; désactivez-le avec `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` et relancez Warp. La Dictée de macOS peut aussi être configurée pour démarrer avec fn.

**Après une mise à jour, Lowspeak ne réagit plus à fn.** macOS lie l’autorisation Accessibilité à la signature de l’app. Si la signature a changé, par exemple en passant d’une version compilée par vos soins à l’app téléchargée, retirez Lowspeak de la liste Accessibilité (sous macOS 27 : Contrôle de l’appareil et accès aux données) et ajoutez-le de nouveau.

**Les premiers mots manquent.** Commencez à parler dès que l’icône d’enregistrement apparaît. Avec des AirPods, le micro a besoin d’une à deux secondes ; un micro intégré ou USB choisi sous **Micro** est prêt presque immédiatement.

**« Micro silencieux ».** Le micro n’a fourni aucun son pendant cinq secondes. Avec des AirPods, un autre appareil utilise peut-être leur micro ; passez au micro intégré ou reconnectez-les. Un MacBook dont l’écran est rabattu coupe aussi le micro intégré.

**Les dictées sont vides.** Le volume d’entrée est peut-être trop bas : des apps comme Teams, Zoom ou Meet le modifient pendant les appels, et macOS le conserve pour chaque appareil. Augmentez-le dans **Réglages Système › Son › Entrée**.

**Le texte a atterri au mauvais endroit.** **Dictées récentes** dans le menu le copie à nouveau dans le presse-papiers.

**« Aucune clé API ».** Choisissez **Définir la clé API…** dans le menu, ou passez à un modèle local.

**La première transcription locale prend du temps.** Chaque modèle est préparé pour votre Mac lors de sa première utilisation ; ensuite, il se charge en une fraction de seconde.

**Lowspeak se ferme juste après son lancement.** Une autre copie de Lowspeak est déjà en cours d’exécution ; quittez-la d’abord.

**Aucune notification n’apparaît.** Autorisez-les dans **Réglages Système › Notifications › Lowspeak**.

**Autre chose.** Les journaux dans `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) décrivent ce qui s’est passé, sans le texte que vous avez dicté. Joignez les lignes pertinentes à un [signalement de problème](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Pour les développeurs

La compilation, les tests, l’organisation du code et les règles qu’il suit sont décrits dans [docs/development.md](../../../docs/development.md) ; les contributions, dans [CONTRIBUTING.md](../../../CONTRIBUTING.md).

### Composants tiers

| Composant | Version | Utilisé pour | Licence |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) via [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | Whisper en local sur le GPU (Metal) | MIT ; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | décodage de Parakeet et fusion des enregistrements longs, traduits en Rust dans `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) et ses crates de frameworks | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation et d’autres frameworks système | MIT ; Zlib, Apache-2.0 ou MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | capture du micro via CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | rééchantillonnage à 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) avec rustls | 3.5 | téléchargement des modèles | MIT ou Apache-2.0 ; rustls et ring sous ISC et Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | sommes de contrôle des modèles téléchargés | MIT ou Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | clés API dans le trousseau | MIT ou Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, erreurs, le vocabulaire de Parakeet | MIT ou Apache-2.0 ; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | mises à jour de l’app installée depuis l’image disque | MIT, avec des parties sous licences BSD et zlib |

Les textes de licence de tout ce que contient l’app sont livrés avec elle dans `Contents/Resources/Acknowledgements.txt`.

Les modèles ne font pas partie de Lowspeak ; il les télécharge quand vous le demandez :

| Modèle | Source | Licence |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (le Whisper d’OpenAI au format ggml) | MIT |
| Parakeet v2, Parakeet Ultra | conversions CoreML par [FluidInference](https://huggingface.co/FluidInference) du Parakeet TDT de NVIDIA | CC-BY-4.0 |

## Licence

Lowspeak est placé sous l’une ou l’autre des licences suivantes

- Licence Apache, version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- Licence MIT ([LICENSE-MIT](../../../LICENSE-MIT))

à votre choix.

Sauf indication contraire de votre part, toute contribution soumise intentionnellement pour inclusion dans Lowspeak par vous, telle que définie dans la licence Apache-2.0, est placée sous la double licence ci-dessus, sans conditions supplémentaires.
