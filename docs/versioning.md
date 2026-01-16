# 📦 Stratégie de Versioning — SemVer

> Ce document définit la stratégie de versioning pour le projet Origis.

---

## Format

Origis suit le [Semantic Versioning 2.0.0](https://semver.org/) :

```
MAJOR.MINOR.PATCH[-PRERELEASE]
```

| Composant | Quand incrémenter | Exemple |
|-----------|-------------------|---------|
| **MAJOR** | Breaking changes, incompatibilité API | 1.0.0 → 2.0.0 |
| **MINOR** | Nouvelle feature (rétrocompatible) | 1.0.0 → 1.1.0 |
| **PATCH** | Bug fix uniquement | 1.0.0 → 1.0.1 |

---

## Règles d'incrémentation

### MAJOR (X.0.0)

Incrémenter quand :
- ❌ Une commande CLI change de syntaxe
- ❌ Un format de fichier devient incompatible
- ❌ Une feature est supprimée
- ❌ Le comportement par défaut change

**Exemple :**
```bash
# v1.x : origis commit -m "message"
# v2.0 : origis save --message "message"  ← Breaking change
```

### MINOR (X.Y.0)

Incrémenter quand :
- ✅ Nouvelle commande ajoutée
- ✅ Nouvelle option/flag
- ✅ Nouveau format supporté
- ✅ Amélioration de performance significative

**Exemple :**
```bash
# v1.0 : origis init, origis commit, origis log
# v1.1 : + origis diff  ← Nouvelle feature
```

### PATCH (X.Y.Z)

Incrémenter quand :
- 🐛 Bug fix
- 📝 Amélioration de message d'erreur
- 🔧 Fix de typo dans la doc
- ⚡ Micro-optimisation

**Règle :** Le PATCH reset à 0 à chaque changement de MINOR ou MAJOR.

---

## Versions Pre-release

Avant une release stable, utiliser des suffixes :

| Suffixe | Usage | Stabilité |
|---------|-------|-----------|
| `-alpha.N` | Développement actif, features incomplètes | 🔴 Instable |
| `-beta.N` | Feature-complete, tests en cours | 🟠 Risqué |
| `-rc.N` | Release Candidate, prêt sauf bugs | 🟡 Quasi-stable |

**Exemple de progression :**
```
0.1.0-alpha.1 → 0.1.0-alpha.2 → 0.1.0-beta.1 → 0.1.0-rc.1 → 0.1.0
```

---

## Versions 0.x.x vs 1.x.x

| Phase | Versions | Signification |
|-------|----------|---------------|
| **Développement** | 0.x.x | Utilisable mais instable, API peut changer |
| **Stable** | 1.x.x+ | Production-ready, breaking changes = major |

Origis sera en **0.x.x** jusqu'à ce que le MVP soit complet et testé.

---

## Processus de Release

1. **Mettre à jour `Cargo.toml`** avec la nouvelle version
2. **Mettre à jour `CHANGELOG.md`**
3. **Créer un tag Git** : `git tag v0.1.0`
4. **Push le tag** : `git push origin v0.1.0`
5. **Créer la Release GitHub** avec notes de version

---

## CHANGELOG

Format basé sur [Keep a Changelog](https://keepachangelog.com/) :

```markdown
## [Unreleased]

### Added
- 

### Changed
- 

### Fixed
- 

### Removed
- 

## [0.1.0] - 2026-XX-XX

### Added
- Initial release
```
