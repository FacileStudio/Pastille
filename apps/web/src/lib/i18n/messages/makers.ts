import type { Row } from "../row";

export const makers = {
  "makers.title": ["For makers", "Pour les makers", "Para makers", "Für Maker"],
  "makers.line": [
    "Open hardware, with schematics, firmware and the bill of materials on GitHub. " +
      "Open source, and you can run it yourself: see self-hosting.",
    "Matériel ouvert, avec schémas, firmware et nomenclature sur GitHub. " +
      "Open source, installable chez vous : voir l'auto-hébergement.",
    "Hardware abierto, con esquemas, firmware y lista de materiales en GitHub. " +
      "Código abierto, y puedes instalarlo tú mismo: ver autoalojamiento.",
    "Offene Hardware, mit Schaltplänen, Firmware und Stückliste auf GitHub. " +
      "Open Source, auch selbst betreibbar: siehe Self-Hosting.",
  ],
  "makers.source": [
    "Schematics, enclosure, bill of materials and firmware. Hardware under CERN-OHL-S v2, firmware under Apache 2.0.",
    "Schémas, boîtier, nomenclature et firmware. Matériel sous CERN-OHL-S v2, firmware sous Apache 2.0.",
    "Esquemas, carcasa, lista de materiales y firmware. Hardware bajo CERN-OHL-S v2, firmware bajo Apache 2.0.",
    "Schaltpläne, Gehäuse, Stückliste und Firmware. Hardware unter CERN-OHL-S v2, Firmware unter Apache 2.0.",
  ],
  "makers.firmware.title": ["Firmware", "Firmware", "Firmware", "Firmware"],
  "makers.firmware": [
    "Bare-metal Rust on Embassy for the ESP32-S3, tested on your computer before it runs on the board.",
    "Rust bare-metal sur Embassy pour l'ESP32-S3, testé sur votre ordinateur avant de tourner sur la carte.",
    "Rust bare-metal sobre Embassy para el ESP32-S3, probado en tu ordenador antes de ir a la placa.",
    "Bare-Metal-Rust auf Embassy für den ESP32-S3, auf dem Rechner getestet, bevor es auf der Platine läuft.",
  ],
  "makers.spec.title": ["BOHS-01 protocol", "Protocole BOHS-01", "Protocolo BOHS-01", "BOHS-01-Protokoll"],
  "makers.spec": [
    "The open spec for devices that work with Bulle: framing, pairing and the tools Bulle can call.",
    "La spécification ouverte des appareils qui travaillent avec Bulle : trames, jumelage et outils que Bulle appelle.",
    "La especificación abierta de los dispositivos que funcionan con Bulle: tramas, vinculación y herramientas.",
    "Die offene Spezifikation für Geräte mit Bulle: Frames, Kopplung und die Werkzeuge, die Bulle aufruft.",
  ],
  "makers.fusible.title": ["Fusible project", "Projet Fusible", "Proyecto Fusible", "Fusible-Projekt"],
  "makers.fusible": [
    "The board is a Fusible project: open it, check it and export the fab files yourself.",
    "La carte est un projet Fusible : ouvrez-la, vérifiez-la et exportez vous-même les fichiers de fabrication.",
    "La placa es un proyecto Fusible: ábrela, verifícala y exporta tú mismo los archivos de fabricación.",
    "Die Platine ist ein Fusible-Projekt: öffnen, prüfen und die Fertigungsdateien selbst exportieren.",
  ],
} satisfies Record<string, Row>;
