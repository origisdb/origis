# ✅ Definition of Done

> Ce document définit les critères à remplir pour qu'une issue soit considérée comme "Done".

---

## Critères Généraux

Tous les types d'issues doivent respecter ces critères :

- [ ] Le code compile sans erreur (`cargo build`)
- [ ] Aucun warning Clippy (`cargo clippy`)
- [ ] Le code est formaté (`cargo fmt`)
- [ ] Les tests passent (`cargo test`)
- [ ] La PR a été reviewée et approuvée (1 minimum)
- [ ] Les conversations de review sont résolues
- [ ] Le code est mergé dans `main`
- [ ] L'issue Linear est passée en "Livré"

---

## Critères par Type

### `type:feature`

- [ ] La fonctionnalité correspond aux critères d'acceptation de l'issue
- [ ] Des tests unitaires couvrent la nouvelle feature
- [ ] La documentation utilisateur est mise à jour (si applicable)
- [ ] Le CHANGELOG est mis à jour

### `type:bug`

- [ ] Le bug est corrigé et ne se reproduit plus
- [ ] Un test de régression est ajouté
- [ ] La cause root est documentée dans la PR

### `type:doc`

- [ ] Le document est clair et sans fautes
- [ ] Les liens fonctionnent
- [ ] Le document est accessible (lié depuis README ou CONTRIBUTING)

### `type:task`

- [ ] La tâche est complètement terminée
- [ ] Les livrables sont documentés/accessibles

### `type:research`

- [ ] Les findings sont documentés
- [ ] Les recommandations sont claires
- [ ] Les sources sont citées

---

## Ce qui n'est PAS "Done"

- ❌ "Ça marche sur ma machine" — doit passer la CI
- ❌ PR ouverte mais pas reviewée
- ❌ Code mergé mais issue pas fermée
- ❌ Feature sans tests
- ❌ Documentation obsolète

---

## Checklist PR

À inclure dans chaque PR :

```markdown
## ✅ Checklist

- [ ] Mon code compile (`cargo build`)
- [ ] J'ai lancé `cargo fmt` et `cargo clippy`
- [ ] Les tests passent (`cargo test`)
- [ ] J'ai ajouté des tests si nécessaire
- [ ] La documentation est à jour
- [ ] J'ai lié l'issue (`Closes #X`)
```

---

## Exceptions

- **Phase 0.x.x** : Les tests peuvent être allégés pendant l'apprentissage Rust
- **Documentation pure** : Les critères de code (build, clippy, tests) ne s'appliquent pas
