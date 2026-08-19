// include/uapi/asm-generic/fcntl.h

// #define O_ACCMODE	3
// #define O_RDONLY	0
// #define O_WRONLY	(1 << 0)
// #define O_RDWR		(1 << 1)
// #define O_CREAT		(1 << 6)	/* not fcntl */
// #define O_EXCL		(1 << 7)	/* not fcntl */
// #define O_NOCTTY	(1 << 8)	/* not fcntl */
// #define O_TRUNC		(1 << 9)	/* not fcntl */
// #define O_APPEND	(1 << 10)
pub const O_NONBLOCK: i32 = 1 << 11;
// #define O_DSYNC		(1 << 12)	/* used to be O_SYNC, see below */
// #define FASYNC		(1 << 13)	/* fcntl, for BSD compatibility */
// #define O_DIRECT	(1 << 14)	/* direct disk access hint */
// #define O_LARGEFILE	(1 << 15)
// #define O_DIRECTORY	(1 << 16)	/* must be a directory */
// #define O_NOFOLLOW	(1 << 17)	/* don't follow links */
// #define O_NOATIME	(1 << 18)
pub const O_CLOEXEC: i32 = 1 << 19;
