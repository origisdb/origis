# 🌿 Convention de Nommage des Branches

> Ce document définit le format de nommage des branches pour le projet Origis.

---

## Format

```
<type>/<issue-number>-<short-description>
```

### Exemples

```bash
feat/5-cargo-init
fix/42-snapshot-crash
docs/15-readme
refactor/core-cleanup
```

---

## Types de branches

| Type | Usage | Exemple |
|------|-------|---------|
| `feat/` | Nouvelle fonctionnalité | `feat/5-cargo-init` |
| `fix/` | Correction de bug | `fix/42-snapshot-crash` |
| `docs/` | Documentation | `docs/15-readme` |
| `refactor/` | Refactoring | `refactor/core-cleanup` |
| `test/` | Ajout de tests | `test/diff-unit-tests` |
| `chore/` | Maintenance, CI | `chore/update-deps` |
| `hotfix/` | Fix urgent sur main | `hotfix/critical-security` |

---

## Règles

| Règle | ✅ Correct | ❌ Incorrect |
|-------|-----------|-------------|
| **Langue** | Anglais | Français |
| **Format** | kebab-case | snake_case ou camelCase |
| **Longueur** | Max 50 caractères | Trop long |
| **Numéro d'issue** | Toujours si issue existe | Oublié |
| **Majuscules** | Aucune | FEAT/5-Init |

---

## Exemples

### ✅ Correct

```bash
git checkout -b feat/5-cargo-init
git checkout -b fix/23-empty-table-crash
git checkout -b docs/16-rust-conventions
git checkout -b chore/update-dependencies
```

### ❌ Incorrect

```bash
git checkout -b new-feature              # Pas de type
git checkout -b FEAT/5-Init              # Majuscules
git checkout -b feat/implement_the_new_init_command  # Trop long + underscores
git checkout -b feature/5                # Description manquante
```

---

## Branches protégées

| Branche | Règle |
|---------|-------|
| `main` | ❌ Jamais de push direct — uniquement via PR |

> Nous n'utilisons pas de branche `develop`. Le travail se fait directement vers `main` via PR.

---

## Cycle de vie

```
1. Créer la branche    → git checkout -b feat/5-cargo-init
2. Développer          → commits...
3. Push                → git push -u origin feat/5-cargo-init
4. Créer la PR         → GitHub
5. Review + Merge      → Squash or Merge
6. Supprimer           → GitHub supprime automatiquement (ou manuellement)
```

---

## Rappels

1. **Toujours inclure le numéro d'issue** si elle existe
2. **Supprimer les branches** après merge
3. **Une branche = une issue** — pas de branches multi-features
