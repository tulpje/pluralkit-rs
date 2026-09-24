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
    Client, RequestBuilder, StatusCode,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue},
};

use crate::{
    models::{
        Group, GroupRef, MemberGuildSettings, Message, PluralKitUuid, Switch, SwitchWithMembers,
        marker::SwitchMarker,
    },
    queue::PluralKitQueue,
    rate_limiter::handle_ratelimit_headers,
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
    pub async fn get_system(&self, system_ref: SystemRef) -> Result<System, Error> {
        self.send_json(
            self.client
                .get(format!("{}/systems/{system_ref}", self.base_url)),
        )
        .await
    }

    pub async fn update_system(&self, system: System) -> Result<System, Error> {
        todo!()
    }

    pub async fn get_my_system_settings(&self) -> Result<SystemSettings, Error> {
        todo!()
    }

    pub async fn get_system_settings(
        &self,
        system_ref: SystemRef,
    ) -> Result<PublicSystemSettings, Error> {
        todo!()
    }

    pub fn update_system_settings(
        &self,
        settings: SystemSettings,
    ) -> Result<SystemSettings, Error> {
        todo!()
    }

    pub fn get_system_guild_settings(
        &self,
        guild_id: String,
    ) -> Result<SystemGuildSettings, Error> {
        todo!()
    }

    pub fn update_system_guild_settings(
        &self,
        settings: SystemGuildSettings,
    ) -> Result<SystemGuildSettings, Error> {
        todo!()
    }

    pub async fn get_system_autoproxy_settings(&self) -> Result<AutoproxySettings, Error> {
        todo!()
    }

    pub fn update_system_autoproxy_settings(
        &self,
        guild_id: String,
        settings: AutoproxySettings,
    ) -> Result<AutoproxySettings, Error> {
        todo!()
    }

    // member
    pub async fn get_system_members(&self, system_ref: SystemRef) -> Result<Vec<Member>, Error> {
        todo!()
    }

    pub async fn create_member(&self, member: Member) -> Result<Member, Error> {
        todo!()
    }

    pub async fn get_member(&self, member_ref: MemberRef) -> Result<Member, Error> {
        todo!()
    }

    pub async fn update_member(&self, member: Member) -> Result<Member, Error> {
        todo!()
    }

    pub async fn delete_member(&self, member_ref: MemberRef) -> Result<(), Error> {
        todo!()
    }

    pub async fn get_member_groups(&self, member_ref: MemberRef) -> Result<Vec<Group>, Error> {
        todo!()
    }

    pub fn add_member_to_groups(
        &self,
        member_ref: MemberRef,
        groups: Vec<GroupRef>,
    ) -> Result<(), Error> {
        todo!()
    }

    pub fn remove_member_from_groups(
        &self,
        member_ref: MemberRef,
        groups: Vec<GroupRef>,
    ) -> Result<(), Error> {
        todo!()
    }

    pub fn overwrite_member_groups(
        &self,
        member_ref: MemberRef,
        groups: Vec<GroupRef>,
    ) -> Result<(), Error> {
        todo!()
    }

    pub fn get_member_guild_settings(
        &self,
        member_ref: MemberRef,
        guild_id: String,
    ) -> Result<MemberGuildSettings, Error> {
        todo!()
    }

    pub fn update_member_guild_settings(
        &self,
        member_ref: MemberRef,
        guild_id: String,
        settings: MemberGuildSettings,
    ) -> Result<MemberGuildSettings, Error> {
        todo!()
    }

    // group
    pub async fn get_system_groups(&self, system_ref: SystemRef) -> Result<Vec<Group>, Error> {
        todo!()
    }

    pub async fn create_group(&self, group: Group) -> Result<Group, Error> {
        todo!()
    }

    pub async fn get_group(&self, group_ref: GroupRef) -> Result<Group, Error> {
        todo!()
    }

    pub async fn update_group(&self, group: Group) -> Result<Group, Error> {
        todo!()
    }

    pub async fn delete_group(&self, group_ref: GroupRef) -> Result<(), Error> {
        todo!()
    }

    pub async fn get_group_members(&self, group_ref: GroupRef) -> Result<Vec<Member>, Error> {
        todo!()
    }

    pub fn add_members_to_group(
        &self,
        group_ref: GroupRef,
        member_refs: Vec<MemberRef>,
    ) -> Result<(), Error> {
        todo!()
    }

    pub fn remove_members_from_group(
        &self,
        group_ref: GroupRef,
        member_refs: Vec<MemberRef>,
    ) -> Result<(), Error> {
        todo!()
    }

    pub fn overwrite_group_members(
        &self,
        group_ref: GroupRef,
        member_refs: Vec<MemberRef>,
    ) -> Result<(), Error> {
        todo!()
    }

    // switches
    pub async fn get_system_switches(&self, system_ref: SystemRef) -> Result<Vec<Switch>, Error> {
        todo!()
    }

    pub fn get_current_system_fronters(
        &self,
        system_ref: SystemRef,
    ) -> Result<SwitchWithMembers, Error> {
        todo!()
    }

    pub fn create_switch(
        &self,
        system_ref: &SystemRef,
        members: Vec<MemberRef>,
        timestamp: Option<DateTime>,
    ) -> Result<SwitchWithMembers, Error> {
        todo!()
    }

    pub fn get_switch(
        &self,
        system_ref: &SystemRef,
        switch_ref: PluralKitUuid<SwitchMarker>,
    ) -> Result<SwitchWithMembers, Error> {
        todo!()
    }

    pub fn update_switch(
        &self,
        system_ref: &SystemRef,
        switch_ref: PluralKitUuid<SwitchMarker>,
        timestamp: DateTime,
    ) -> Result<SwitchWithMembers, Error> {
        todo!()
    }

    pub fn update_switch_members(
        &self,
        system_ref: &SystemRef,
        switch_ref: PluralKitUuid<SwitchMarker>,
        members: Vec<MemberRef>,
    ) -> Result<SwitchWithMembers, Error> {
        todo!()
    }

    pub async fn delete_switch(
        &self,
        system_ref: &SystemRef,
        switch_ref: PluralKitUuid<SwitchMarker>,
    ) -> Result<(), Error> {
        todo!()
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
