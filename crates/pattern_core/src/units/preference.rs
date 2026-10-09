/// Unit preference storage and resolution (AC-FR-038-4, OQ-35).
///
/// Resolution order (OQ-35 decision, 2026-10-09):
/// 1. Project scope + specific quantity class
/// 2. Global scope + specific quantity class
/// 3. Project scope + no class (project default)
/// 4. Global scope + no class (global default)
/// 5. PROVISIONAL per-class default from [`QuantityClass::provisional_default_unit`]
use crate::units::{QuantityClass, Scope, Unit};

/// A unit preference record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitPreference {
    /// The scope this preference applies to.
    pub scope: Scope,
    /// The quantity class, or `None` for a scope-wide default.
    pub class: Option<QuantityClass>,
    /// The preferred unit.
    pub unit: Unit,
}

/// Holds the set of active preferences and resolves the display unit for a
/// given (class, scope) query.
///
/// `scope` in `resolve` means "resolve as if the caller is in this scope".
/// Project-scope preferences are visible when `scope == Project`.
/// Global-scope preferences are always visible.
#[derive(Debug, Default)]
pub struct PreferenceStore {
    prefs: Vec<UnitPreference>,
}

impl PreferenceStore {
    /// Create an empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set (or replace) a preference.  Replaces any existing entry with the same
    /// scope+class key.
    pub fn set(&mut self, pref: UnitPreference) {
        // Remove existing entry with same key.
        self.prefs
            .retain(|p| !(p.scope == pref.scope && p.class == pref.class));
        self.prefs.push(pref);
    }

    /// Resolve the display unit for `class` in `scope`.
    ///
    /// See module-level doc for the resolution order.
    pub fn resolve(&self, class: QuantityClass, scope: Scope) -> Unit {
        // 1. Project + class (only if scope is Project)
        if scope == Scope::Project {
            if let Some(u) = self.lookup(Scope::Project, Some(class)) {
                return u;
            }
        }
        // 2. Global + class
        if let Some(u) = self.lookup(Scope::Global, Some(class)) {
            return u;
        }
        // 3. Project default (only if scope is Project)
        if scope == Scope::Project {
            if let Some(u) = self.lookup(Scope::Project, None) {
                return u;
            }
        }
        // 4. Global default
        if let Some(u) = self.lookup(Scope::Global, None) {
            return u;
        }
        // 5. Provisional built-in default
        class.provisional_default_unit()
    }

    fn lookup(&self, scope: Scope, class: Option<QuantityClass>) -> Option<Unit> {
        self.prefs
            .iter()
            .find(|p| p.scope == scope && p.class == class)
            .map(|p| p.unit)
    }
}
