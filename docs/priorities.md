# 🎯 Système de Priorités P0-P3

> Ce document définit les niveaux de priorité utilisés pour les issues du projet Origis.

---

## Vue d'ensemble

| Priorité | Nom | SLA | Couleur |
|----------|-----|-----|---------|
| **P0** | 🔴 Urgent / Bloquant | < 24h | `#b60205` |
| **P1** | 🟠 Important | < 1 semaine | `#d93f0b` |
| **P2** | 🟡 Normal | Sprint courant | `#fbca04` |
| **P3** | 🟢 Nice to have | Quand possible | `#0e8a16` |

---

## P0 — 🔴 Urgent / Bloquant

**Définition :** Problème critique qui bloque le projet ou l'équipe.

### Critères

- L'application ne fonctionne plus (crash, erreur fatale)
- Un membre de l'équipe est bloqué et ne peut pas avancer
- Deadline critique imminente
- Faille de sécurité découverte

### Exemples

- ❌ Le build CI est cassé, personne ne peut merger
- ❌ La branche main est corrompue
- ❌ Vulnérabilité critique dans une dépendance

### SLA

Traitement sous **24h maximum**, souvent immédiat.

---

## P1 — 🟠 Important

**Définition :** Issue importante qui impacte significativement le projet.

### Critères

- Fonctionnalité core du MVP
- Bug visible par les utilisateurs
- Documentation essentielle manquante
- Dépendance pour d'autres issues

### Exemples

- Implémenter la commande `origis init`
- Corriger un bug dans le système de versioning
- Setup de la CI (bloque protection de branche)

### SLA

Planifié dans le sprint en cours, livré sous **1 semaine**.

---

## P2 — 🟡 Normal

**Définition :** Issue standard, fait partie du flux normal de travail.

### Critères

- Feature non-bloquante
- Amélioration de l'existant
- Documentation complémentaire
- Refactoring mineur

### Exemples

- Améliorer les messages d'erreur
- Ajouter des tests supplémentaires
- Documenter une fonctionnalité existante

### SLA

Planifié dans le sprint courant, livré **selon capacité**.

---

## P3 — 🟢 Nice to Have

**Définition :** Issue souhaitable mais non prioritaire.

### Critères

- Amélioration cosmétique
- Feature "bonus" non essentielle
- Idée à explorer plus tard
- Dette technique non urgente

### Exemples

- Ajouter des couleurs dans le CLI
- Optimisation de performance mineure
- Refactoring "clean code" non urgent

### SLA

**Pas de deadline**, traité quand le backlog le permet.

---

## Labels GitHub

| Label | Couleur |
|-------|---------|
| `priority:p0` | 🔴 `#b60205` |
| `priority:p1` | 🟠 `#d93f0b` |
| `priority:p2` | 🟡 `#fbca04` |
| `priority:p3` | 🟢 `#0e8a16` |

---

## Mapping Linear

| GitHub | Linear |
|--------|--------|
| `priority:p0` | 🔴 Urgent |
| `priority:p1` | 🟠 High |
| `priority:p2` | 🟡 Medium |
| `priority:p3` | 🟢 Low |

> ⚠️ **Note :** Les labels GitHub sont informatifs. Pour filtrer/trier dans Linear, utiliser les priorités natives de Linear.

---

## Règles

1. **Une priorité par issue** — Pas de multi-priorité
2. **Le PO assigne la priorité** — Après analyse
3. **Éviter l'inflation P0** — Être strict sur les critères
4. **Réviser si nécessaire** — La priorité peut changer
