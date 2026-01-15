# 📝 Convention de Commits

> Ce document définit le format des messages de commit pour le projet Origis.

---

## Format

Nous utilisons le format [Conventional Commits](https://www.conventionalcommits.org/) :

```
<type>(<scope>): <description>

[body optionnel]

[footer optionnel]
```

---

## Types de commits

| Type | Usage | Exemple |
|------|-------|---------|
| `feat` | Nouvelle fonctionnalité | `feat(cli): add origis diff command` |
| `fix` | Correction de bug | `fix(snapshot): handle empty tables` |
| `docs` | Documentation | `docs: add commit conventions` |
| `style` | Formatage (pas de changement de code) | `style: run rustfmt` |
| `refactor` | Refactoring (pas de nouvelle feature/fix) | `refactor(core): extract snapshot logic` |
| `test` | Ajout/modification de tests | `test: add unit tests for diff` |
| `chore` | Maintenance, CI, deps | `chore: update dependencies` |
| `ci` | Changements CI/CD | `ci: add clippy to workflow` |

---

## Règles de rédaction

| Règle | ✅ Bon | ❌ Mauvais |
|-------|--------|-----------|
| **Langue** | Anglais | Français |
| **Temps** | Impératif présent | Passé |
| **Longueur titre** | Max 72 caractères | Trop long |
| **Ponctuation** | Pas de point final | Point à la fin. |
| **Majuscule** | Minuscule après le `:` | Majuscule après `:` |

### Exemples

```bash
# ✅ Correct
feat(cli): add origis init command
fix: resolve crash on empty database
docs: update README with installation steps

# ❌ Incorrect
feat(cli): Added origis init command.   # passé + point
Fix: Resolve crash                       # majuscule après :
nouvelle feature pour init               # français + pas de type
```

---

## Scope (optionnel)

Le scope précise la partie du code affectée :

| Scope | Description |
|-------|-------------|
| `cli` | Interface en ligne de commande |
| `core` | Logique métier centrale |
| `snapshot` | Système de snapshots |
| `db` | Connexion base de données |
| `config` | Configuration |

---

## Body (optionnel)

Utilisez le body pour expliquer **pourquoi** ce changement :

```
fix(snapshot): handle empty tables correctly

Previously, an empty table would cause a panic during snapshot.
This fix adds a check to skip empty tables gracefully.
```

---

## Footer — Lien avec les issues

Utilisez `Closes #X` pour fermer automatiquement une issue :

```
feat(cli): add origis init command

Implements the basic initialization command that creates
the .origis directory and config file.

Closes #5
```

### Mots-clés reconnus par GitHub

- `Closes #X`
- `Fixes #X`
- `Resolves #X`

---

## Exemples complets

### Feature simple

```
feat(cli): add origis status command
```

### Bug fix avec explication

```
fix(db): prevent connection leak on error

The database connection was not being closed when an error
occurred during snapshot creation. Added proper cleanup.

Fixes #42
```

### Documentation

```
docs: add contribution guidelines
```

### Breaking change

```
feat(cli)!: rename snapshot to commit

BREAKING CHANGE: The `origis snapshot` command is now `origis commit`
to align with git terminology.

Closes #78
```

---

## Rappels

1. **Un commit = un changement logique** — Pas de commits "fourre-tout"
2. **Commits atomiques** — Chaque commit doit compiler
3. **Relire avant de push** — `git log --oneline` pour vérifier
