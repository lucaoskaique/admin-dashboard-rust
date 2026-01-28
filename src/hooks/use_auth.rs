use crate::api::UserType;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

const USER_STORAGE_KEY: &str = "angel_admin_user";

#[derive(Clone)]
pub struct AuthContext {
    pub user: Option<UserType>,
    pub login: Callback<UserType>,
    pub logout: Callback<()>,
}

impl PartialEq for AuthContext {
    fn eq(&self, other: &Self) -> bool {
        // Compare only the user field (callbacks can't be compared)
        match (&self.user, &other.user) {
            (None, None) => true,
            (Some(a), Some(b)) => a.id == b.id && a.email == b.email,
            _ => false,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let user = use_state(|| LocalStorage::get::<UserType>(USER_STORAGE_KEY).ok());

    let login = {
        let user = user.clone();
        Callback::from(move |new_user: UserType| {
            let _ = LocalStorage::set(USER_STORAGE_KEY, &new_user);
            user.set(Some(new_user));
        })
    };

    let logout = {
        let user = user.clone();
        Callback::from(move |_| {
            LocalStorage::delete(USER_STORAGE_KEY);
            user.set(None);
            // Note: HTTP-only cookies can only be cleared by the server
        })
    };

    let context = AuthContext {
        user: (*user).clone(),
        login,
        logout,
    };

    html! {
        <ContextProvider<AuthContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<AuthContext>>
    }
}

#[hook]
pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>().expect("AuthContext not found")
}
