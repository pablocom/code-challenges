//! A small social-network feed: users share insights and follow each other.
//! A user's feed shows, in share order, the insights from everyone reachable
//! through their (transitive) connections — including themselves.

use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct SocialNetwork {
    insights: Vec<i32>,
    insights_by_user: HashMap<i32, Vec<i32>>,
    connections_by_user: HashMap<i32, HashSet<i32>>,
}

impl SocialNetwork {
    const MAX_INSIGHTS: usize = 10;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn share_insight(&mut self, user_id: i32, insight_id: i32) {
        self.insights_by_user
            .entry(user_id)
            .or_default()
            .push(insight_id);
        self.insights.push(insight_id);
    }

    pub fn add_connection(&mut self, follower_id: i32, followed_id: i32) {
        self.connections_by_user
            .entry(follower_id)
            .or_default()
            .insert(followed_id);
    }

    pub fn remove_connection(&mut self, follower_id: i32, unfollowed_id: i32) {
        if let Some(connections) = self.connections_by_user.get_mut(&follower_id) {
            connections.remove(&unfollowed_id);
        }
    }

    pub fn last_insights(&self, user_id: i32) -> Vec<i32> {
        let connected = self.connected_users(user_id);
        let visible: HashSet<i32> = self.insights_of(&connected);

        let mut feed = Vec::with_capacity(Self::MAX_INSIGHTS);
        for &insight in &self.insights {
            if visible.contains(&insight) {
                feed.push(insight);
                if feed.len() == Self::MAX_INSIGHTS {
                    break;
                }
            }
        }
        feed
    }

    /// Everyone reachable from `user_id` via connections, including `user_id`.
    fn connected_users(&self, user_id: i32) -> HashSet<i32> {
        let mut visited = HashSet::new();
        let mut stack = vec![user_id];

        while let Some(user) = stack.pop() {
            if !visited.insert(user) {
                continue;
            }
            if let Some(connections) = self.connections_by_user.get(&user) {
                stack.extend(connections.iter().copied());
            }
        }

        visited
    }

    fn insights_of(&self, users: &HashSet<i32>) -> HashSet<i32> {
        let mut result = HashSet::new();
        for user in users {
            if let Some(insights) = self.insights_by_user.get(user) {
                result.extend(insights.iter().copied());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shows_own_insight() {
        let mut network = SocialNetwork::new();
        network.share_insight(12, 69);
        assert_eq!(network.last_insights(12), vec![69]);
    }

    #[test]
    fn shows_insights_in_share_order() {
        let mut network = SocialNetwork::new();
        network.share_insight(12, 69);
        network.share_insight(12, 70);
        assert_eq!(network.last_insights(12), vec![69, 70]);
    }

    #[test]
    fn shows_insights_from_connected_users() {
        let mut network = SocialNetwork::new();
        network.add_connection(13, 12);
        network.share_insight(12, 69);
        network.share_insight(13, 70);
        network.share_insight(12, 71);
        network.share_insight(14, 72);
        assert_eq!(network.last_insights(13), vec![69, 70, 71]);
    }

    #[test]
    fn empty_feed_when_no_insights() {
        let network = SocialNetwork::new();
        assert!(network.last_insights(12).is_empty());
    }

    #[test]
    fn removing_a_connection_hides_its_insights() {
        let mut network = SocialNetwork::new();
        network.add_connection(12, 13);
        network.share_insight(12, 69);
        network.share_insight(13, 70);
        network.share_insight(12, 71);
        network.remove_connection(12, 13);
        assert_eq!(network.last_insights(12), vec![69, 71]);
    }
}
