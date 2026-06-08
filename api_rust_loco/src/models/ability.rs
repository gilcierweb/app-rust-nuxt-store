use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Select};

use super::_entities::users as users_entity;
use super::users;

pub type Attribute = &'static str;

const USER_ATTRIBUTES: &[Attribute] = &[
    "id",
    "pid",
    "email",
    "name",
    "role",
    "active",
    "email_verified_at",
    "created_at",
    "updated_at",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Action {
    Manage,
    Read,
    Create,
    Update,
    Destroy,
    Index,
    Show,
    New,
    Edit,
    Delete,
    List,
}

impl Action {
    #[must_use]
    pub fn matches(self, requested: Self) -> bool {
        self == Self::Manage
            || self == requested
            || matches!(
                (self, requested),
                (Self::Read, Self::Index | Self::Show | Self::List)
                    | (Self::Create, Self::New)
                    | (Self::Update, Self::Edit)
                    | (Self::Destroy, Self::Delete)
            )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Subject {
    All,
    Admin,
    User,
    Dashboard,
    Product,
    Category,
    Order,
    Review,
    Banner,
    Coupon,
    Shipping,
    Shipment,
    Profile,
    Address,
    Post,
    Variant,
    Payment,
    Inventory,
    Settings,
    AuditLog,
    Email,
    Rbac,
}

impl Subject {
    #[must_use]
    pub fn matches(self, requested: Self) -> bool {
        self == Self::All || self == requested
    }

    #[must_use]
    pub fn section_name(self) -> &'static str {
        match self {
            Self::Dashboard => "dashboard",
            Self::Product => "products",
            Self::Category => "categories",
            Self::Order => "orders",
            Self::Review => "reviews",
            Self::Banner => "banners",
            Self::Coupon => "coupons",
            Self::Shipping => "shippings",
            Self::Shipment => "shipments",
            Self::Profile => "profiles",
            Self::Address => "addresses",
            Self::Post => "posts",
            Self::Variant => "variants",
            Self::Payment => "payments",
            Self::Inventory => "inventory",
            Self::Settings => "settings",
            Self::AuditLog => "audit_logs",
            Self::Email => "emails",
            Self::Rbac => "rbac",
            Self::User => "customers",
            Self::Admin => "admin",
            Self::All => "all",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resource {
    Admin,
    User { id: i32 },
}

impl Resource {
    #[must_use]
    pub fn subject(self) -> Subject {
        match self {
            Self::Admin => Subject::Admin,
            Self::User { .. } => Subject::User,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Condition {
    Any,
    UserId(i32),
}

impl Condition {
    fn matches_resource(self, resource: Option<Resource>) -> bool {
        match self {
            Self::Any => true,
            Self::UserId(user_id) => {
                matches!(resource, Some(Resource::User { id }) if id == user_id)
            }
        }
    }

    fn matches_class(self) -> bool {
        self == Self::Any
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Rule {
    allowed: bool,
    actions: Vec<Action>,
    subjects: Vec<Subject>,
    condition: Condition,
    attributes: Vec<Attribute>,
}

impl Rule {
    fn can(actions: Vec<Action>, subjects: Vec<Subject>) -> Self {
        Self::new(true, actions, subjects, Condition::Any, Vec::new())
    }

    fn can_with_condition(
        actions: Vec<Action>,
        subjects: Vec<Subject>,
        condition: Condition,
    ) -> Self {
        Self::new(true, actions, subjects, condition, Vec::new())
    }

    fn can_with_attributes(
        actions: Vec<Action>,
        subjects: Vec<Subject>,
        attributes: Vec<Attribute>,
    ) -> Self {
        Self::new(true, actions, subjects, Condition::Any, attributes)
    }

    fn cannot(actions: Vec<Action>, subjects: Vec<Subject>) -> Self {
        Self::new(false, actions, subjects, Condition::Any, Vec::new())
    }

    fn cannot_with_condition(
        actions: Vec<Action>,
        subjects: Vec<Subject>,
        condition: Condition,
    ) -> Self {
        Self::new(false, actions, subjects, condition, Vec::new())
    }

    fn new(
        allowed: bool,
        actions: Vec<Action>,
        subjects: Vec<Subject>,
        condition: Condition,
        attributes: Vec<Attribute>,
    ) -> Self {
        Self {
            allowed,
            actions,
            subjects,
            condition,
            attributes,
        }
    }

    fn matches_class(&self, action: Action, subject: Subject) -> bool {
        self.matches_action_and_subject(action, subject) && self.condition.matches_class()
    }

    fn matches_resource(&self, action: Action, resource: Resource) -> bool {
        self.matches_action_and_subject(action, resource.subject())
            && self.condition.matches_resource(Some(resource))
    }

    fn matches_attribute(&self, action: Action, subject: Subject) -> bool {
        self.matches_action_and_subject(action, subject) && self.condition.matches_class()
    }

    fn matches_action_and_subject(&self, action: Action, subject: Subject) -> bool {
        self.actions
            .iter()
            .any(|rule_action| rule_action.matches(action))
            && self
                .subjects
                .iter()
                .any(|rule_subject| rule_subject.matches(subject))
    }
}

#[derive(Clone, Debug, Default)]
pub struct Ability {
    rules: Vec<Rule>,
}

impl Ability {
    #[must_use]
    pub fn from_roles(roles: Vec<String>) -> Self {
        Self::for_roles_and_user(roles, None)
    }

    #[must_use]
    pub fn for_roles_and_user(roles: Vec<String>, user_id: Option<i32>) -> Self {
        let mut ability = Self::default();

        if let Some(user_id) = user_id {
            ability.define_authenticated_user_rules(user_id);
        }

        let has_admin = roles.iter().any(|role| role == "admin");
        let has_store_manager = roles.iter().any(|role| role == "store_manager");
        let has_editor = roles.iter().any(|role| role == "editor");
        let has_support = roles.iter().any(|role| role == "support");
        let has_viewer = roles.iter().any(|role| role == "viewer");

        if has_admin {
            ability.define_admin_rules(user_id);
        }
        if has_store_manager {
            ability.define_store_manager_rules();
        }
        if has_editor {
            ability.define_editor_rules();
        }
        if has_support {
            ability.define_support_rules();
        }
        if has_viewer {
            ability.define_viewer_rules();
        }

        ability
    }

    pub async fn for_user(user: &users::Model, db: &DatabaseConnection) -> ModelResult<Self> {
        Ok(Self::for_roles_and_user(
            user.roles(db).await?,
            Some(user.id),
        ))
    }

    pub async fn for_user_pid(
        db: &DatabaseConnection,
        pid: &str,
    ) -> ModelResult<(users::Model, Self)> {
        let user = users::Model::find_by_pid(db, pid).await?;
        let ability = Self::for_user(&user, db).await?;

        Ok((user, ability))
    }

    pub fn allow(&mut self, actions: Vec<Action>, subjects: Vec<Subject>) {
        self.rules.push(Rule::can(actions, subjects));
    }

    pub fn allow_attributes(
        &mut self,
        actions: Vec<Action>,
        subjects: Vec<Subject>,
        attributes: Vec<Attribute>,
    ) {
        self.rules
            .push(Rule::can_with_attributes(actions, subjects, attributes));
    }

    pub fn deny(&mut self, actions: Vec<Action>, subjects: Vec<Subject>) {
        self.rules.push(Rule::cannot(actions, subjects));
    }

    pub fn allow_user_id(&mut self, actions: Vec<Action>, subjects: Vec<Subject>, user_id: i32) {
        self.rules.push(Rule::can_with_condition(
            actions,
            subjects,
            Condition::UserId(user_id),
        ));
    }

    pub fn deny_user_id(&mut self, actions: Vec<Action>, subjects: Vec<Subject>, user_id: i32) {
        self.rules.push(Rule::cannot_with_condition(
            actions,
            subjects,
            Condition::UserId(user_id),
        ));
    }

    #[must_use]
    pub fn can(&self, action: Action, subject: Subject) -> bool {
        self.rules
            .iter()
            .rev()
            .find(|rule| rule.matches_class(action, subject))
            .map(|rule| rule.allowed)
            .unwrap_or(false)
    }

    #[must_use]
    pub fn can_resource(&self, action: Action, resource: Resource) -> bool {
        self.rules
            .iter()
            .rev()
            .find(|rule| rule.matches_resource(action, resource))
            .map(|rule| rule.allowed)
            .unwrap_or(false)
    }

    #[must_use]
    pub fn cannot(&self, action: Action, subject: Subject) -> bool {
        !self.can(action, subject)
    }

    #[must_use]
    pub fn cannot_resource(&self, action: Action, resource: Resource) -> bool {
        !self.can_resource(action, resource)
    }

    pub fn authorize(&self, action: Action, subject: Subject) -> Result<()> {
        if self.can(action, subject) {
            Ok(())
        } else {
            Err(Error::Unauthorized(t!("auth.unauthorized").to_string()))
        }
    }

    pub fn authorize_resource(&self, action: Action, resource: Resource) -> Result<()> {
        if self.can_resource(action, resource) {
            Ok(())
        } else {
            Err(Error::Unauthorized(t!("auth.unauthorized").to_string()))
        }
    }

    #[must_use]
    pub fn accessible_users_query(
        &self,
        action: Action,
        current_user_id: i32,
    ) -> Select<users_entity::Entity> {
        let query = users_entity::Entity::find();

        if self.can(action, Subject::User) {
            query
        } else if self.can_resource(
            action,
            Resource::User {
                id: current_user_id,
            },
        ) {
            query.filter(users_entity::Column::Id.eq(current_user_id))
        } else {
            query.filter(users_entity::Column::Id.eq(-1))
        }
    }

    #[must_use]
    pub fn permitted_attributes(&self, action: Action, subject: Subject) -> Vec<Attribute> {
        let mut attributes = Vec::new();

        for rule in self
            .rules
            .iter()
            .filter(|rule| rule.matches_attribute(action, subject))
        {
            if rule.allowed {
                for attribute in &rule.attributes {
                    if !attributes.contains(attribute) {
                        attributes.push(*attribute);
                    }
                }
            } else if rule.attributes.is_empty() {
                attributes.clear();
            } else {
                attributes.retain(|attribute| !rule.attributes.contains(attribute));
            }
        }

        attributes
    }

    #[must_use]
    pub fn can_manage_admin(&self) -> bool {
        self.can(Action::Manage, Subject::Admin)
    }

    #[must_use]
    pub fn can_access_section(&self, subject: Subject) -> bool {
        self.can(Action::Read, subject) || self.can(Action::Manage, subject)
    }

    #[must_use]
    pub fn admin_sections(&self) -> Vec<&'static str> {
        let sections = [
            Subject::Dashboard,
            Subject::Product,
            Subject::Category,
            Subject::Order,
            Subject::Review,
            Subject::Banner,
            Subject::Coupon,
            Subject::Shipping,
            Subject::Shipment,
            Subject::Profile,
            Subject::Address,
            Subject::Post,
            Subject::Variant,
            Subject::Payment,
            Subject::Inventory,
            Subject::Settings,
            Subject::AuditLog,
            Subject::Email,
            Subject::Rbac,
            Subject::User,
        ];

        sections
            .iter()
            .filter(|&&s| self.can_access_section(s))
            .map(|s| s.section_name())
            .collect()
    }

    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.rules.extend(other.rules);
        self
    }

    fn define_authenticated_user_rules(&mut self, user_id: i32) {
        self.allow_user_id(
            vec![Action::Read, Action::Update],
            vec![Subject::User],
            user_id,
        );
    }

    fn define_admin_rules(&mut self, user_id: Option<i32>) {
        self.allow(vec![Action::Manage], vec![Subject::All]);
        self.allow_attributes(
            vec![Action::Manage],
            vec![Subject::User],
            USER_ATTRIBUTES.to_vec(),
        );

        if let Some(user_id) = user_id {
            self.deny_user_id(vec![Action::Destroy], vec![Subject::User], user_id);
        }
    }

    fn define_store_manager_rules(&mut self) {
        self.allow(vec![Action::Manage], vec![Subject::Admin]);
        self.allow(
            vec![Action::Read],
            vec![
                Subject::Dashboard,
                Subject::User,
                Subject::AuditLog,
            ],
        );
        self.allow(
            vec![Action::Manage],
            vec![
                Subject::Product,
                Subject::Category,
                Subject::Order,
                Subject::Review,
                Subject::Banner,
                Subject::Coupon,
                Subject::Shipping,
                Subject::Shipment,
                Subject::Variant,
                Subject::Inventory,
            ],
        );
        self.allow(
            vec![Action::Read],
            vec![
                Subject::Payment,
                Subject::Address,
                Subject::Profile,
            ],
        );
    }

    fn define_editor_rules(&mut self) {
        self.allow(vec![Action::Manage], vec![Subject::Admin]);
        self.allow(
            vec![Action::Read],
            vec![
                Subject::Dashboard,
                Subject::Payment,
                Subject::Shipment,
                Subject::Shipping,
                Subject::Address,
                Subject::Profile,
                Subject::User,
            ],
        );
        self.allow(
            vec![Action::Manage],
            vec![
                Subject::Product,
                Subject::Category,
                Subject::Review,
                Subject::Banner,
                Subject::Post,
                Subject::Variant,
            ],
        );
    }

    fn define_support_rules(&mut self) {
        self.allow(vec![Action::Manage], vec![Subject::Admin]);
        self.allow(
            vec![Action::Read],
            vec![
                Subject::Dashboard,
                Subject::Product,
                Subject::Category,
                Subject::Order,
                Subject::Payment,
                Subject::Shipment,
                Subject::Shipping,
                Subject::Address,
                Subject::Profile,
                Subject::User,
                Subject::Review,
                Subject::Coupon,
                Subject::Variant,
            ],
        );
        self.allow(
            vec![Action::Update],
            vec![Subject::Review],
        );
    }

    fn define_viewer_rules(&mut self) {
        self.allow(vec![Action::Manage], vec![Subject::Admin]);
        self.allow(
            vec![Action::Read],
            vec![
                Subject::Dashboard,
                Subject::Product,
                Subject::Category,
                Subject::Order,
                Subject::Review,
                Subject::Banner,
                Subject::Coupon,
                Subject::Shipping,
                Subject::Shipment,
                Subject::Profile,
                Subject::Address,
                Subject::Post,
                Subject::Variant,
                Subject::Payment,
                Subject::Inventory,
                Subject::AuditLog,
                Subject::User,
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{Ability, Action, Resource, Subject};

    #[test]
    fn denies_by_default() {
        let ability = Ability::default();

        assert!(ability.cannot(Action::Read, Subject::Admin));
        assert!(ability.cannot_resource(Action::Read, Resource::User { id: 1 }));
    }

    #[test]
    fn supports_admin_manage_all() {
        let ability = Ability::for_roles_and_user(vec!["admin".to_string()], Some(1));

        assert!(ability.can(Action::Read, Subject::Admin));
        assert!(ability.can(Action::Edit, Subject::User));
        assert!(ability.can_resource(Action::Update, Resource::User { id: 2 }));
    }

    #[test]
    fn supports_action_aliases() {
        let ability = Ability::for_roles_and_user(vec!["admin".to_string()], Some(1));

        assert!(ability.can(Action::Index, Subject::User));
        assert!(ability.can(Action::Show, Subject::User));
        assert!(ability.can(Action::New, Subject::User));
        assert!(ability.can(Action::Delete, Subject::User));
    }

    #[test]
    fn supports_resource_conditions() {
        let ability = Ability::for_roles_and_user(vec!["user".to_string()], Some(7));

        assert!(ability.cannot(Action::Read, Subject::User));
        assert!(ability.can_resource(Action::Read, Resource::User { id: 7 }));
        assert!(ability.cannot_resource(Action::Read, Resource::User { id: 8 }));
    }

    #[test]
    fn later_cannot_rule_overrides_previous_can_rule() {
        let ability = Ability::for_roles_and_user(vec!["admin".to_string()], Some(7));

        assert!(ability.cannot_resource(Action::Destroy, Resource::User { id: 7 }));
        assert!(ability.can_resource(Action::Destroy, Resource::User { id: 8 }));
    }

    #[test]
    fn exposes_permitted_attributes() {
        let ability = Ability::for_roles_and_user(vec!["admin".to_string()], Some(1));

        assert!(ability
            .permitted_attributes(Action::Read, Subject::User)
            .contains(&"email"));
    }

    #[test]
    fn supports_composed_abilities_and_public_deny_rules() {
        let mut base = Ability::default();
        base.allow(vec![Action::Read], vec![Subject::User]);

        let mut override_rules = Ability::default();
        override_rules.deny(vec![Action::Read], vec![Subject::User]);

        let ability = base.merge(override_rules);

        assert!(ability.cannot(Action::Read, Subject::User));
    }

    #[test]
    fn store_manager_can_manage_products() {
        let ability = Ability::for_roles_and_user(vec!["store_manager".to_string()], None);

        assert!(ability.can(Action::Manage, Subject::Product));
        assert!(ability.can(Action::Manage, Subject::Category));
        assert!(ability.can(Action::Manage, Subject::Order));
        assert!(ability.can(Action::Manage, Subject::Inventory));
        assert!(ability.can(Action::Read, Subject::Dashboard));
    }

    #[test]
    fn store_manager_cannot_manage_users() {
        let ability = Ability::for_roles_and_user(vec!["store_manager".to_string()], None);

        assert!(ability.cannot(Action::Manage, Subject::User));
        assert!(ability.cannot(Action::Manage, Subject::Rbac));
        assert!(ability.cannot(Action::Manage, Subject::Settings));
    }

    #[test]
    fn editor_can_manage_content() {
        let ability = Ability::for_roles_and_user(vec!["editor".to_string()], None);

        assert!(ability.can(Action::Manage, Subject::Post));
        assert!(ability.can(Action::Manage, Subject::Banner));
        assert!(ability.can(Action::Manage, Subject::Review));
        assert!(ability.can(Action::Manage, Subject::Product));
        assert!(ability.can(Action::Read, Subject::Dashboard));
    }

    #[test]
    fn editor_cannot_manage_orders() {
        let ability = Ability::for_roles_and_user(vec!["editor".to_string()], None);

        assert!(ability.cannot(Action::Manage, Subject::Order));
        assert!(ability.cannot(Action::Manage, Subject::Coupon));
        assert!(ability.cannot(Action::Manage, Subject::Shipping));
    }

    #[test]
    fn support_can_read_most_things() {
        let ability = Ability::for_roles_and_user(vec!["support".to_string()], None);

        assert!(ability.can(Action::Read, Subject::Order));
        assert!(ability.can(Action::Read, Subject::Product));
        assert!(ability.can(Action::Read, Subject::User));
        assert!(ability.can(Action::Read, Subject::Payment));
        assert!(ability.can(Action::Update, Subject::Review));
    }

    #[test]
    fn support_cannot_manage_products() {
        let ability = Ability::for_roles_and_user(vec!["support".to_string()], None);

        assert!(ability.cannot(Action::Manage, Subject::Product));
        assert!(ability.cannot(Action::Manage, Subject::Order));
        assert!(ability.cannot(Action::Destroy, Subject::User));
    }

    #[test]
    fn viewer_can_only_read() {
        let ability = Ability::for_roles_and_user(vec!["viewer".to_string()], None);

        assert!(ability.can(Action::Read, Subject::Dashboard));
        assert!(ability.can(Action::Read, Subject::Product));
        assert!(ability.can(Action::Read, Subject::Order));
        assert!(ability.can(Action::Read, Subject::AuditLog));
    }

    #[test]
    fn viewer_cannot_write_anything() {
        let ability = Ability::for_roles_and_user(vec!["viewer".to_string()], None);

        assert!(ability.cannot(Action::Create, Subject::Product));
        assert!(ability.cannot(Action::Update, Subject::Order));
        assert!(ability.cannot(Action::Destroy, Subject::Review));
        assert!(ability.cannot(Action::Manage, Subject::Settings));
    }

    #[test]
    fn admin_sections_returns_accessible_sections() {
        let ability = Ability::for_roles_and_user(vec!["admin".to_string()], Some(1));
        let sections = ability.admin_sections();
        assert!(sections.contains(&"dashboard"));
        assert!(sections.contains(&"products"));
        assert!(sections.contains(&"rbac"));
        assert!(sections.contains(&"settings"));
    }

    #[test]
    fn viewer_sections_exclude_management() {
        let ability = Ability::for_roles_and_user(vec!["viewer".to_string()], None);
        let sections = ability.admin_sections();
        assert!(sections.contains(&"dashboard"));
        assert!(sections.contains(&"products"));
        assert!(!sections.contains(&"settings"));
        assert!(!sections.contains(&"rbac"));
        assert!(!sections.contains(&"emails"));
    }

    #[test]
    fn composite_roles_merge_permissions() {
        let ability = Ability::for_roles_and_user(
            vec!["editor".to_string(), "support".to_string()],
            None,
        );

        assert!(ability.can(Action::Manage, Subject::Post));
        assert!(ability.can(Action::Read, Subject::Order));
        assert!(ability.can(Action::Update, Subject::Review));
    }

    #[test]
    fn subject_section_name_mapping() {
        assert_eq!(Subject::Dashboard.section_name(), "dashboard");
        assert_eq!(Subject::Product.section_name(), "products");
        assert_eq!(Subject::Order.section_name(), "orders");
        assert_eq!(Subject::Rbac.section_name(), "rbac");
        assert_eq!(Subject::Inventory.section_name(), "inventory");
        assert_eq!(Subject::Payment.section_name(), "payments");
    }
}
