pub mod models;
pub mod rate_limiter;
pub mod request;

mod queue;

use std::{sync::Arc, time::Duration};

use jiff::civil::DateTime;
use models::{
    AutoproxySettings, Member, MemberRef, PublicSystemSettings, System, SystemGuildSettings,
    SystemRef, SystemSettings,
};
use reqwest::{
    Client, Method, StatusCode,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue, InvalidHeaderValue},
};
use serde_json::json;

use crate::{
    models::{
        Group, GroupRef, MemberGuildSettings, Message, PluralKitUuid, Switch, SwitchWithMembers,
        group::GroupWithMembers, marker::SwitchMarker, switch::CreateSwitch,
    },
    queue::PluralKitQueue,
    rate_limiter::handle_ratelimit_headers,
    request::{EmptyBody, Request},
};

type Error = Box<dyn std::error::Error + Send + Sync>;

const FALLBACK_WAIT_DURATION: Duration = Duration::from_secs(2);

pub struct PluralKit {
    client: Client,
    base_url: String,
    queue: Arc<PluralKitQueue>,
}

impl Default for PluralKit {
    fn default() -> Self {
        Self::new()
    }
}

impl PluralKit {
    pub fn new() -> Self {
        let queue = Arc::new(PluralKitQueue::new());
        let runner = PluralKitRunner::new(queue.clone());
        tokio::spawn(async move {
            runner.run().await;
        });

        Self {
            client: Client::builder()
                .user_agent(format!("pluralkit-rs/{}", env!("CARGO_PKG_VERSION")))
                .default_headers(HeaderMap::from_iter([(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                )]))
                .build()
                .expect("error building reqwest client"),
            base_url: String::from("https://api.pluralkit.me/v2"),
            queue,
        }
    }

    // system
    pub fn get_system(&self, system_ref: &SystemRef) -> Request<System> {
        self.get(format!("/systems/{system_ref}"))
    }

    pub fn update_system(&self, system_ref: &SystemRef, system: System) -> Request<System> {
        self.patch(format!("/systems/{system_ref}")).json(&system)
    }

    pub fn get_my_system_settings(
        &self,
        token: impl Into<String>,
    ) -> Result<Request<SystemSettings>, InvalidHeaderValue> {
        self.get("/systems/@me/settings").token(token)
    }

    pub fn get_system_settings(&self, system_ref: &SystemRef) -> Request<PublicSystemSettings> {
        self.get(format!("/systems/{system_ref}/settings"))
    }

    pub fn update_system_settings(
        &self,
        system_ref: &SystemRef,
        settings: SystemSettings,
        token: impl Into<String>,
    ) -> Result<Request<SystemSettings>, InvalidHeaderValue> {
        self.patch(format!("/systems/{system_ref}/settings"))
            .json(&settings)
            .token(token)
    }

    pub fn get_system_guild_settings(
        &self,
        guild_id: String,
        token: impl Into<String>,
    ) -> Result<Request<SystemGuildSettings>, InvalidHeaderValue> {
        self.get(format!("/systems/@me/guilds/{guild_id}"))
            .token(token)
    }

    pub fn update_system_guild_settings(
        &self,
        guild_id: String,
        settings: SystemGuildSettings,
        token: impl Into<String>,
    ) -> Result<Request<SystemGuildSettings>, InvalidHeaderValue> {
        self.patch(format!("/systems/@me/guilds/{guild_id}"))
            .json(&settings)
            .token(token)
    }

    pub fn get_system_autoproxy_settings(
        &self,
        guild_id: String,
        token: impl Into<String>,
    ) -> Result<Request<AutoproxySettings>, InvalidHeaderValue> {
        self.get("/systems/@me/autoproxy")
            .query("guild_id", guild_id)
            .token(token)
    }

    pub fn update_system_autoproxy_settings(
        &self,
        guild_id: String,
        settings: AutoproxySettings,
        token: impl Into<String>,
    ) -> Result<Request<AutoproxySettings>, InvalidHeaderValue> {
        self.patch("/systems/@me/autoproxy")
            .query("guild_id", guild_id)
            .json(&settings)
            .token(token)
    }

    // member
    pub fn get_system_members(&self, system_ref: &SystemRef) -> Request<Vec<Member>> {
        self.get(format!("/systems/{system_ref}/members"))
    }

    pub fn create_member(
        &self,
        member: Member,
        token: impl Into<String>,
    ) -> Result<Request<Member>, InvalidHeaderValue> {
        self.post("/members").json(&member).token(token)
    }

    pub fn get_member(&self, member_ref: MemberRef) -> Request<Member> {
        self.get(format!("/members/{member_ref}"))
    }

    pub fn update_member(
        &self,
        member_ref: MemberRef,
        member: Member,
        token: impl Into<String>,
    ) -> Result<Request<Member>, InvalidHeaderValue> {
        self.patch(format!("/members/{member_ref}"))
            .json(&member)
            .token(token)
    }

    pub fn delete_member(
        &self,
        member_ref: MemberRef,
        token: impl Into<String>,
    ) -> Result<Request<EmptyBody>, InvalidHeaderValue> {
        self.delete(format!("/members/{member_ref}")).token(token)
    }

    pub fn get_member_groups(&self, member_ref: MemberRef) -> Request<Vec<Group>> {
        self.get(format!("/members/{member_ref}/groups"))
    }

    pub fn add_member_to_groups(
        &self,
        member_ref: MemberRef,
        groups: Vec<GroupRef>,
        token: impl Into<String>,
    ) -> Result<Request<EmptyBody>, InvalidHeaderValue> {
        self.post(format!("/members/{member_ref}/groups/add"))
            .json(&groups)
            .token(token)
    }

    pub fn remove_member_from_groups(
        &self,
        member_ref: MemberRef,
        groups: Vec<GroupRef>,
        token: impl Into<String>,
    ) -> Result<Request<EmptyBody>, InvalidHeaderValue> {
        self.post(format!("/members/{member_ref}/groups/remove"))
            .json(&groups)
            .token(token)
    }

    pub fn overwrite_member_groups(
        &self,
        member_ref: MemberRef,
        groups: Vec<GroupRef>,
        token: impl Into<String>,
    ) -> Result<Request<EmptyBody>, InvalidHeaderValue> {
        self.post(format!("/members/{member_ref}/groups/overwrite"))
            .json(&groups)
            .token(token)
    }

    pub fn get_member_guild_settings(
        &self,
        member_ref: MemberRef,
        guild_id: String,
    ) -> Request<MemberGuildSettings> {
        self.get(format!("/members/{member_ref}/guilds/{guild_id}"))
    }

    pub fn update_member_guild_settings(
        &self,
        member_ref: MemberRef,
        guild_id: String,
        settings: MemberGuildSettings,
        token: impl Into<String>,
    ) -> Result<Request<MemberGuildSettings>, InvalidHeaderValue> {
        self.patch(format!("/members/{member_ref}/guilds/{guild_id}"))
            .json(&settings)
            .token(token)
    }

    // group
    pub fn get_system_groups(&self, system_ref: &SystemRef) -> Request<Vec<Group>> {
        self.get(format!("/systems/{system_ref}/groups"))
    }

    pub fn get_system_groups_with_members(
        &self,
        system_ref: &SystemRef,
    ) -> Request<Vec<GroupWithMembers>> {
        self.get(format!("/systems/{system_ref}/groups"))
            .query("with_members", "true")
    }

    pub fn create_group(
        &self,
        group: Group,
        token: impl Into<String>,
    ) -> Result<Request<Group>, InvalidHeaderValue> {
        self.post("/groups").json(&group).token(token)
    }

    pub fn get_group(&self, group_ref: GroupRef) -> Request<Group> {
        self.get(format!("/groups/{group_ref}"))
    }

    pub fn update_group(
        &self,
        group_ref: GroupRef,
        group: Group,
        token: impl Into<String>,
    ) -> Result<Request<Group>, InvalidHeaderValue> {
        self.patch(format!("/groups/{group_ref}"))
            .json(&group)
            .token(token)
    }

    pub fn delete_group(
        &self,
        group_ref: GroupRef,
        token: impl Into<String>,
    ) -> Result<Request<EmptyBody>, InvalidHeaderValue> {
        self.delete(format!("/groups/{group_ref}")).token(token)
    }

    pub fn get_group_members(&self, group_ref: GroupRef) -> Request<Vec<Member>> {
        self.get(format!("/groups/{group_ref}/members"))
    }

    pub fn add_members_to_group(
        &self,
        group_ref: GroupRef,
        member_refs: Vec<MemberRef>,
        token: impl Into<String>,
    ) -> Result<Request<EmptyBody>, InvalidHeaderValue> {
        self.post(format!("/groups/{group_ref}/members/add"))
            .json(&member_refs)
            .token(token)
    }

    pub fn remove_members_from_group(
        &self,
        group_ref: GroupRef,
        member_refs: Vec<MemberRef>,
        token: impl Into<String>,
    ) -> Result<Request<EmptyBody>, InvalidHeaderValue> {
        self.post(format!("/groups/{group_ref}/members/remove"))
            .json(&member_refs)
            .token(token)
    }

    pub fn overwrite_group_members(
        &self,
        group_ref: GroupRef,
        member_refs: Vec<MemberRef>,
        token: impl Into<String>,
    ) -> Result<Request<EmptyBody>, InvalidHeaderValue> {
        self.post(format!("/groups/{group_ref}/members/overwrite"))
            .json(&member_refs)
            .token(token)
    }

    // switches
    pub fn get_system_switches(&self, system_ref: &SystemRef) -> Request<Vec<Switch>> {
        self.get(format!("/systems/{system_ref}/switches"))
    }

    pub fn get_current_system_fronters(
        &self,
        system_ref: &SystemRef,
    ) -> Request<SwitchWithMembers> {
        self.get(format!("/systems/{system_ref}/fronters"))
    }

    pub fn create_switch(
        &self,
        system_ref: &SystemRef,
        members: Vec<MemberRef>,
        timestamp: Option<DateTime>,
        token: impl Into<String>,
    ) -> Result<Request<SwitchWithMembers>, InvalidHeaderValue> {
        self.post(format!("/systems/{system_ref}/switches"))
            .json(&CreateSwitch::new(members, timestamp))
            .token(token)
    }

    pub fn get_switch(
        &self,
        system_ref: &SystemRef,
        switch_ref: PluralKitUuid<SwitchMarker>,
    ) -> Request<SwitchWithMembers> {
        self.get(format!("/systems/{system_ref}/switches/{switch_ref}"))
    }

    pub fn update_switch(
        &self,
        system_ref: &SystemRef,
        switch_ref: PluralKitUuid<SwitchMarker>,
        timestamp: DateTime,
        token: impl Into<String>,
    ) -> Result<Request<SwitchWithMembers>, InvalidHeaderValue> {
        self.patch(format!("/systems/{system_ref}/switches/{switch_ref}"))
            .json(&json!({"timestamp": timestamp}))
            .token(token)
    }

    pub fn update_switch_members(
        &self,
        system_ref: &SystemRef,
        switch_ref: PluralKitUuid<SwitchMarker>,
        members: Vec<MemberRef>,
        token: impl Into<String>,
    ) -> Result<Request<SwitchWithMembers>, InvalidHeaderValue> {
        self.patch(format!(
            "/systems/{system_ref}/switches/{switch_ref}/members"
        ))
        .json(&json!({"members": members}))
        .token(token)
    }

    pub fn delete_switch(
        &self,
        system_ref: &SystemRef,
        switch_ref: PluralKitUuid<SwitchMarker>,
        token: impl Into<String>,
    ) -> Result<Request<EmptyBody>, InvalidHeaderValue> {
        self.delete(format!("/systems/{system_ref}/switches/{switch_ref}"))
            .token(token)
    }

    // misc
    pub async fn get_proxied_message_information(
        &self,
        message_id: String,
    ) -> Result<Message, Error> {
        todo!()
    }

    // shorthand
    fn get<T: Send + Sync>(&self, path: impl Into<String>) -> Request<T> {
        self.request(Method::GET, path)
    }
    fn post<T: Send + Sync>(&self, path: impl Into<String>) -> Request<T> {
        self.request(Method::POST, path)
    }
    fn patch<T: Send + Sync>(&self, path: impl Into<String>) -> Request<T> {
        self.request(Method::PATCH, path)
    }
    fn delete<T: Send + Sync>(&self, path: impl Into<String>) -> Request<T> {
        self.request(Method::DELETE, path)
    }

    fn request<T: Send + Sync>(&self, method: Method, path: impl Into<String>) -> Request<T> {
        Request::new(
            &self.client,
            method,
            &self.base_url,
            &path.into(),
            self.queue.clone(),
        )
    }
}

struct PluralKitRunner {
    queue: Arc<PluralKitQueue>,
}

impl PluralKitRunner {
    fn new(queue: Arc<PluralKitQueue>) -> Self {
        Self { queue }
    }

    async fn run(&self) {
        loop {
            let (prio, request) = self.queue.pop().await;

            let resp = if let Some(cloned_req) = request.req.try_clone() {
                let resp = cloned_req.send().await;

                // status 429 retry logic
                if let Ok(ref resp) = resp
                    && resp.status() == StatusCode::TOO_MANY_REQUESTS
                {
                    // requeue with same priority to retry after any higher priority items
                    self.queue.retry(request, prio);

                    // sleep for rate limit
                    let sleep_duration =
                        handle_ratelimit_headers(resp.headers()).unwrap_or(FALLBACK_WAIT_DURATION);
                    tokio::time::sleep(sleep_duration).await;

                    continue;
                }

                resp
            } else {
                // can only send once no matter what
                request.req.send().await
            };

            // calculate sleep duration from headers and status code
            let sleep_duration = resp.as_ref().map_or(None, |resp| {
                handle_ratelimit_headers(resp.headers()).or_else(|| {
                    (resp.status() == StatusCode::TOO_MANY_REQUESTS)
                        .then_some(FALLBACK_WAIT_DURATION)
                })
            });

            // send response (or error) back to client
            if request.response_tx.send(resp).is_err() {
                println!("ERR: couldnt sending response back, dropping");
            }

            // wait for ratelimiting after sending response back
            if let Some(sleep_duration) = sleep_duration {
                tokio::time::sleep(sleep_duration).await;
            }
        }
    }
}
