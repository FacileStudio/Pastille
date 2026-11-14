import type { Row } from "../row";

export const demo = {
  "demo.label": [
    "In Bulle, you ask what is left for today. Bulle checks your list in Opus and buzzes your Pastille.",
    "Dans Bulle, vous demandez ce qu'il reste à faire. Bulle consulte votre liste dans Opus et fait vibrer votre " +
      "Pastille.",
    "En Bulle, preguntas qué te queda hoy. Bulle revisa tu lista en Opus y hace vibrar tu Pastille.",
    "In Bulle fragen Sie, was heute noch ansteht. Bulle prüft Ihre Liste in Opus und lässt Ihre Pastille vibrieren.",
  ],
  "demo.pastQuestion": [
    "Is my Pastille charged?",
    "Ma Pastille est chargée ?",
    "¿Está cargada mi Pastille?",
    "Ist meine Pastille geladen?",
  ],
  "demo.pastTool": [
    "Read your Pastille's battery",
    "Batterie de votre Pastille lue",
    "Batería de tu Pastille leída",
    "Akku Ihrer Pastille gelesen",
  ],
  "demo.pastAnswer": [
    "82 %, enough for a few more days.",
    "82 %, assez pour quelques jours encore.",
    "82 %, suficiente para unos días más.",
    "82 %, genug für ein paar weitere Tage.",
  ],
  "demo.question": [
    "What's left on my list for today?",
    "Qu'est-ce qu'il me reste à faire aujourd'hui ?",
    "¿Qué me queda en la lista para hoy?",
    "Was steht heute noch auf meiner Liste?",
  ],
  "demo.toolList": [
    "Checked your list in Opus",
    "Liste consultée dans Opus",
    "Lista revisada en Opus",
    "Liste in Opus geprüft",
  ],
  "demo.toolBuzz": [
    "Buzzed your Pastille",
    "Pastille mise en vibration",
    "Pastille vibrando",
    "Pastille vibrieren lassen",
  ],
  "demo.answer": [
    "Two things: call the plumber, and send Léa the slides before six.",
    "Deux choses : appeler le plombier, et envoyer les slides à Léa avant 18 h.",
    "Dos cosas: llamar al fontanero y enviar las diapositivas a Léa antes de las seis.",
    "Zwei Dinge: den Klempner anrufen und Léa die Folien vor 18 Uhr schicken.",
  ],
  "demo.mood.idle": ["Ready", "Prête", "Lista", "Bereit"],
  "demo.mood.thinking": ["Thinking", "Réfléchit", "Piensa", "Denkt"],
  "demo.mood.buzz": ["Buzz", "Vibre", "Vibra", "Vibriert"],
  "demo.mood.showing": ["For you", "Pour vous", "Para ti", "Für Sie"],
} satisfies Record<string, Row>;
