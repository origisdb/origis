# 📝 User Stories MVP

> Ce document définit les user stories pour le MVP d'Origis.

---

## Personas

| Persona | Description | Besoins |
|---------|-------------|---------|
| **Dev Solo** | Développeur indie/freelance | Simple, rapide, local-first |
| **Petite équipe** | Startup 3-10 devs | Collaboration, branches |
| **Data Engineer** | Travaille avec des datasets | Performance, gros volumes |

---

## User Stories

### Must Have (MVP)

| # | User Story | Priorité |
|---|------------|----------|
| 1 | En tant que dev, je veux **initialiser Origis** sur mon projet afin de commencer à versionner ma base de données | P0 |
| 2 | En tant que dev, je veux **créer un snapshot** de ma BDD afin de sauvegarder son état actuel | P0 |
| 3 | En tant que dev, je veux **voir l'historique** des snapshots afin de naviguer dans les versions | P0 |
| 4 | En tant que dev, je veux **restaurer un snapshot** précédent afin de revenir à un état antérieur | P0 |
| 5 | En tant que dev, je veux **un message d'erreur clair** si quelque chose échoue afin de comprendre le problème | P0 |

### Should Have

| # | User Story | Priorité |
|---|------------|----------|
| 6 | En tant que dev, je veux **voir les différences** entre 2 snapshots afin de comprendre ce qui a changé | P1 |
| 7 | En tant que dev, je veux **ignorer certaines tables** afin de ne pas versionner les données temporaires | P1 |
| 8 | En tant que dev, je veux **voir le statut actuel** (modifié/non modifié) afin de savoir si je dois créer un snapshot | P1 |

### Could Have

| # | User Story | Priorité |
|---|------------|----------|
| 9 | En tant que dev, je veux **créer une branche** afin d'expérimenter sans affecter la branche principale | P2 |
| 10 | En tant que dev, je veux **merger une branche** afin d'intégrer mes changements | P2 |
| 11 | En tant que dev, je veux **exporter un snapshot** afin de le partager avec un collègue | P2 |

### Won't Have (v1)

| # | User Story | Raison |
|---|------------|--------|
| 12 | Support PostgreSQL | Trop complexe pour le MVP, prévu v0.3+ |
| 13 | Interface graphique | CLI first, GUI plus tard |
| 14 | Sync cloud | Local-first pour l'instant |

---

## Critères d'acceptation détaillés

### Story 1 — Initialiser Origis

**Commande :** `origis init`

**Critères :**
- [ ] Crée un dossier `.origis/` à la racine du projet
- [ ] Crée un fichier de config `.origis/config.toml`
- [ ] Détecte automatiquement les fichiers SQLite dans le dossier
- [ ] Affiche un message de succès avec les prochaines étapes
- [ ] Si déjà initialisé, affiche un message d'erreur clair

**Exemple :**
```bash
$ origis init
✅ Origis initialized in ./.origis
📁 Detected databases: ./data.db
💡 Next: origis snapshot -m "Initial snapshot"
```

---

### Story 2 — Créer un snapshot

**Commande :** `origis snapshot -m "message"`

**Critères :**
- [ ] Crée une copie de l'état actuel de la BDD
- [ ] Génère un ID unique pour le snapshot
- [ ] Enregistre le message, la date, et l'auteur
- [ ] Affiche un résumé (tables, lignes, taille)
- [ ] Échoue proprement si la BDD est vide

**Exemple :**
```bash
$ origis snapshot -m "Add users table"
✅ Snapshot created: abc1234
📊 3 tables, 150 rows, 24 KB
```

---

### Story 3 — Voir l'historique

**Commande :** `origis log`

**Critères :**
- [ ] Liste tous les snapshots du plus récent au plus ancien
- [ ] Affiche : ID, message, date, auteur
- [ ] Supporte une limite `--limit N`
- [ ] Format lisible et aligné

**Exemple :**
```bash
$ origis log
abc1234 - Add users table (2 hours ago)
def5678 - Initial snapshot (yesterday)
```

---

### Story 4 — Restaurer un snapshot

**Commande :** `origis restore <snapshot-id>`

**Critères :**
- [ ] Remplace la BDD actuelle par le snapshot
- [ ] Demande confirmation avant d'écraser
- [ ] Option `--force` pour skip la confirmation
- [ ] Affiche un résumé après restauration

**Exemple :**
```bash
$ origis restore abc1234
⚠️  This will overwrite your current database. Continue? [y/N] y
✅ Restored to snapshot abc1234
```

---

## Roadmap

| Version | Stories incluses |
|---------|-----------------|
| **v0.1.0** | #1, #2, #3, #4, #5 |
| **v0.2.0** | #6, #7, #8 |
| **v0.3.0** | #9, #10, #11 + PostgreSQL |
| **v1.0.0** | Stable, production-ready |
