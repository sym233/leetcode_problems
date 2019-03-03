class Twitter {
private:
    typedef unordered_set<int> Following;
    struct Post {
        int userId;
        int tweetId;
        Post(int userId, int tweetId) : userId(userId), tweetId(tweetId) {}
    };
    typedef vector<Post> Tl;
    Tl timeLine;
    typedef unordered_map<int, Following> Followings;
    Followings followings;
public:
    /** Initialize your data structure here. */
    Twitter() {
        timeLine.clear();
        followings.clear();
    }
    
    /** Compose a new tweet. */
    void postTweet(int userId, int tweetId) {
        timeLine.push_back(Post(userId, tweetId));
    }
    
    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    vector<int> getNewsFeed(int userId) {
        vector<int> news;
        Following& following = followings[userId];
        for (Tl::reverse_iterator rit = timeLine.rbegin(); rit != timeLine.rend(); rit++) {
            int poster = rit->userId;
            if (poster == userId || following.find(poster) != following.end()) {
                // my tweet or following this poster
                news.push_back(rit->tweetId);
                if (news.size() >= 10) {
                    break;
                }
            }
        }
        return news;
    }
    
    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    void follow(int followerId, int followeeId) {
        Following& following = followings[followerId];
        following.insert(followeeId);
    }
    
    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    void unfollow(int followerId, int followeeId) {
        Following& following = followings[followerId];
        following.erase(followeeId);
    }
};

/**
 * Your Twitter object will be instantiated and called as such:
 * Twitter obj = new Twitter();
 * obj.postTweet(userId,tweetId);
 * vector<int> param_2 = obj.getNewsFeed(userId);
 * obj.follow(followerId,followeeId);
 * obj.unfollow(followerId,followeeId);
 */
