import type { Row } from "../row";

export const inside = {
  "inside.title": ["What's inside", "Ce qu'il y a dedans", "Lo que hay dentro", "Was drin steckt"],
  "inside.line": [
    "Scroll to take Pastille apart, one layer at a time.",
    "Faites défiler pour démonter Pastille, couche par couche.",
    "Desplázate para desmontar Pastille, capa a capa.",
    "Scrollen Sie, um Pastille Schicht für Schicht zu zerlegen.",
  ],
  "inside.loading": [
    "Loading 3D assembly",
    "Chargement du modèle 3D",
    "Cargando el modelo 3D",
    "3D-Modell wird geladen",
  ],
  "inside.error": [
    "3D preview unavailable",
    "Aperçu 3D indisponible",
    "Vista 3D no disponible",
    "3D-Vorschau nicht verfügbar",
  ],
  "inside.errorLine": [
    "The steps below still describe every part.",
    "Les étapes ci-dessous décrivent quand même chaque pièce.",
    "Los pasos de abajo describen igualmente cada pieza.",
    "Die Schritte unten beschreiben trotzdem jedes Teil.",
  ],
  "inside.steps": ["Disassembly steps", "Étapes du démontage", "Pasos del desmontaje", "Schritte der Zerlegung"],
  "inside.stage": [
    "Pastille disassembly, step {n} of {total}: {title}",
    "Démontage de Pastille, étape {n} sur {total} : {title}",
    "Desmontaje de Pastille, paso {n} de {total}: {title}",
    "Zerlegung von Pastille, Schritt {n} von {total}: {title}",
  ],
  "step.front.title": ["The assembled charm", "Le badge assemblé", "La insignia montada", "Der fertige Anstecker"],
  "step.front.text": [
    "Pastille as you wear it, a 42 mm disc with Bulle on a round screen.",
    "Pastille telle que vous la portez, un disque de 42 mm avec Bulle sur un écran rond.",
    "Pastille tal como la llevas, un disco de 42 mm con Bulle en una pantalla redonda.",
    "Pastille, wie Sie sie tragen: eine 42-mm-Scheibe mit Bulle auf einem runden Display.",
  ],
  "step.bezel.title": ["Lens and bezel", "Lentille et lunette", "Lente y bisel", "Linse und Lünette"],
  "step.bezel.text": [
    "A printed front bezel holds a clear 0.8 mm lens over the screen. Its round window matches the visible area.",
    "Une lunette imprimée tient une lentille claire de 0,8 mm sur l'écran. Sa fenêtre ronde suit la zone visible.",
    "Un bisel impreso sujeta una lente transparente de 0,8 mm sobre la pantalla. Su ventana redonda sigue el área " +
      "visible.",
    "Eine gedruckte Lünette hält eine klare 0,8-mm-Linse über dem Display. Ihr rundes Fenster passt zur sichtbaren " +
      "Fläche.",
  ],
  "step.lcd.title": ["Round LCD", "Écran LCD rond", "LCD redonda", "Rundes LCD"],
  "step.lcd.text": [
    "A 1.28 inch GC9A01 round LCD shows 240x240 pixels. The main board drives it over SPI.",
    "Un écran rond GC9A01 de 1,28 pouce affiche 240x240 pixels. La carte principale le pilote en SPI.",
    "Una LCD redonda GC9A01 de 1,28 pulgadas muestra 240x240 píxeles. La placa principal la controla por SPI.",
    "Ein rundes 1,28-Zoll-LCD GC9A01 zeigt 240x240 Pixel. Die Hauptplatine steuert es über SPI.",
  ],
  "step.pcb.title": ["Main board", "Carte principale", "Placa principal", "Hauptplatine"],
  "step.pcb.text": [
    "A round board carries an ESP32-S3 module with 4 MB of flash, 2 MB of PSRAM, Bluetooth and Wi-Fi. " +
      "It runs bare-metal Rust firmware built on Embassy.",
    "Une carte ronde porte un module ESP32-S3 avec 4 Mo de flash, 2 Mo de PSRAM, Bluetooth et Wi-Fi. " +
      "Elle fait tourner un firmware Rust bare-metal basé sur Embassy.",
    "Una placa redonda lleva un módulo ESP32-S3 con 4 MB de flash, 2 MB de PSRAM, Bluetooth y Wi-Fi. " +
      "Ejecuta un firmware Rust bare-metal basado en Embassy.",
    "Eine runde Platine trägt ein ESP32-S3-Modul mit 4 MB Flash, 2 MB PSRAM, Bluetooth und WLAN. " +
      "Darauf läuft Bare-Metal-Rust-Firmware auf Basis von Embassy.",
  ],
  "step.audio.title": ["Mic and speaker", "Micro et haut-parleur", "Micrófono y altavoz", "Mikrofon und Lautsprecher"],
  "step.audio.text": [
    "An SPH0645LM4H-B MEMS mic feeds the ESP32-S3 over I2S. " +
      "A MAX98357A amplifier takes I2S audio back out and drives a 15 x 11 mm speaker.",
    "Un micro MEMS SPH0645LM4H-B alimente l'ESP32-S3 en I2S. " +
      "Un amplificateur MAX98357A ressort l'audio I2S et pilote un haut-parleur de 15 x 11 mm.",
    "Un micrófono MEMS SPH0645LM4H-B envía audio al ESP32-S3 por I2S. " +
      "Un amplificador MAX98357A saca el audio I2S y mueve un altavoz de 15 x 11 mm.",
    "Ein MEMS-Mikrofon SPH0645LM4H-B liefert Audio per I2S an den ESP32-S3. " +
      "Ein MAX98357A-Verstärker gibt I2S-Audio aus und treibt einen 15 x 11 mm großen Lautsprecher.",
  ],
  "step.haptics.title": ["Haptics", "Vibreur", "Vibración", "Haptik"],
  "step.haptics.text": [
    "A DRV2605L driver runs a small LRA motor for taps and alerts. The ESP32-S3 controls it over I2C.",
    "Un pilote DRV2605L fait tourner un petit moteur LRA pour les tapotements et les alertes. L'ESP32-S3 le pilote " +
      "en I2C.",
    "Un controlador DRV2605L mueve un pequeño motor LRA para toques y alertas. El ESP32-S3 lo controla por I2C.",
    "Ein DRV2605L-Treiber steuert einen kleinen LRA-Motor für Tippen und Hinweise. Der ESP32-S3 steuert ihn über I2C.",
  ],
  "step.battery.title": ["Battery and charger", "Batterie et chargeur", "Batería y cargador", "Akku und Ladegerät"],
  "step.battery.text": [
    "A {capacity} {cell} lithium polymer cell powers the charm. A TP4056 charges it from USB-C at about {current}.",
    "Une batterie lithium polymère {cell} de {capacity} alimente le badge. Un TP4056 la charge en USB-C à environ " +
      "{current}.",
    "Una batería de polímero de litio {cell} de {capacity} alimenta la insignia. Un TP4056 la carga por USB-C a " +
      "unos {current}.",
    "Ein Lithium-Polymer-Akku {cell} mit {capacity} versorgt den Anstecker. Ein TP4056 lädt ihn über USB-C mit etwa " +
      "{current}.",
  ],
  "step.shell.title": ["Back shell", "Coque arrière", "Carcasa trasera", "Rückschale"],
  "step.shell.text": [
    "The printed back shell has the USB-C opening, a port for the mic and a grille for the speaker. " +
      "Two M2 screws close it onto the board's mounting holes.",
    "La coque arrière imprimée a l'ouverture USB-C, un trou pour le micro et une grille pour le haut-parleur. " +
      "Deux vis M2 la ferment sur les trous de fixation de la carte.",
    "La carcasa trasera impresa tiene la abertura USB-C, un orificio para el micrófono " +
      "y una rejilla para el altavoz. Dos tornillos M2 la cierran sobre los orificios de la placa.",
    "Die gedruckte Rückschale hat die USB-C-Öffnung, eine Öffnung fürs Mikrofon und ein Gitter für den Lautsprecher. " +
      "Zwei M2-Schrauben schließen sie an den Befestigungslöchern der Platine.",
  ],
  "board.title": ["The board in 3D", "La carte en 3D", "La placa en 3D", "Die Platine in 3D"],
  "board.line": [
    "Designed and checked with Fusible. Tap Explore in 3D, then drag to turn it.",
    "Dessinée et vérifiée avec Fusible. Touchez Explorer en 3D, puis faites-la tourner.",
    "Diseñada y verificada con Fusible. Toca Explorar en 3D y arrástrala para girarla.",
    "Mit Fusible entworfen und geprüft. Tippen Sie auf In 3D erkunden und drehen Sie sie.",
  ],
  "board.pcb": ["3D board", "Carte 3D", "Placa 3D", "3D-Platine"],
  "board.case": ["Enclosure", "Boîtier", "Carcasa", "Gehäuse"],
  "board.loading": ["Loading 3D geometry", "Chargement de la 3D", "Cargando la 3D", "3D wird geladen"],
  "board.errorLine": [
    "Could not load the 3D model. Reload the page to try again.",
    "Le modèle 3D n'a pas pu être chargé. Rechargez la page pour réessayer.",
    "No se pudo cargar el modelo 3D. Recarga la página para intentarlo de nuevo.",
    "Das 3D-Modell konnte nicht geladen werden. Laden Sie die Seite neu.",
  ],
  "board.explore": ["Explore in 3D", "Explorer en 3D", "Explorar en 3D", "In 3D erkunden"],
  "board.done": ["Done", "Terminé", "Listo", "Fertig"],
  "board.hint": [
    "Drag to orbit · Pinch or scroll to zoom",
    "Glissez pour tourner · Pincez ou faites défiler pour zoomer",
    "Arrastra para girar · Pellizca o desplázate para ampliar",
    "Ziehen zum Drehen · Zoomen mit zwei Fingern oder Mausrad",
  ],
  "board.fuse": [
    "Checked with Fusible · 0 DRC violations",
    "Vérifiée avec Fusible · 0 erreur DRC",
    "Verificada con Fusible · 0 errores DRC",
    "Mit Fusible geprüft · 0 DRC-Fehler",
  ],
} satisfies Record<string, Row>;
