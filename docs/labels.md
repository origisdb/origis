# 🏷️ Système de Labels

> Ce document décrit les labels utilisés sur GitHub et Linear pour le projet Origis.

---

## Labels de Priorité

| Label | Couleur | Hex | Usage |
|-------|---------|-----|-------|
| `priority:p0` | 🔴 Rouge | `#b60205` | Urgent/Bloquant — traitement < 24h |
| `priority:p1` | 🟠 Orange | `#d93f0b` | Important — dans le sprint |
| `priority:p2` | 🟡 Jaune | `#fbca04` | Normal — selon capacité |
| `priority:p3` | 🟢 Vert | `#0e8a16` | Nice to have — quand possible |

---

## Labels de Type

| Label | Couleur | Hex | Usage |
|-------|---------|-----|-------|
| `type:task` | 🟣 Violet | `#5319e7` | Tâche technique |
| `type:bug` | 🔴 Rouge | `#d73a4a` | Bug à corriger |
| `type:feature` | 🔵 Cyan | `#a2eeef` | Nouvelle fonctionnalité |
| `type:doc` | 🔵 Bleu | `#0075ca` | Documentation |
| `type:research` | 💜 Lavande | `#d4c5f9` | Recherche/Exploration |
| `type:design` | 🩷 Rose | `#f9d0c4` | Design/UX |

---

## Mapping GitHub ↔ Linear

### Priorités

| GitHub | Linear |
|--------|--------|
| `priority:p0` | 🔴 Urgent |
| `priority:p1` | 🟠 High |
| `priority:p2` | 🟡 Medium |
| `priority:p3` | 🟢 Low |

> ⚠️ **Important :** Les labels GitHub sont informatifs. Pour filtrer/trier dans Linear, utiliser les **priorités natives de Linear**.

---

## Guidelines

### Quand labelliser ?

| Moment | Action |
|--------|--------|
| À la création | Ajouter le label de type |
| Après analyse | Ajouter le label de priorité |

### Qui labellise ?

| Label | Responsable |
|-------|-------------|
| Type | Créateur de l'issue |
| Priorité | PO (Thomas) ou reviewer |

---

## Règles

1. ✅ **Une issue = 1 type + 1 priorité**
2. ❌ **Pas de multi-types** — si ambigu, choisir le principal
3. ❌ **Pas d'issue sans label type**
4. 🔄 **Labels peuvent évoluer** — ajuster si nécessaire
