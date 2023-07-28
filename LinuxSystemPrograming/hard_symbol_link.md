# Inode
Linux 文件系统会为每个文件分配两个数据结构: 索引节点 (index node) 和目录项 (directory entry), 它们主要用来记录文件的 元信息 和 目录层次结构.
* 索引节点 (inode) 记录文件的元信息. 如 inode 编号、文件大小、访问权限、创建时间、修改时间、数据在磁盘的位置等等. 索引节点是文件的唯一标识, 它们之间一一对应, 也同样都 会被存储在硬盘中, 所以索引节点同样占用磁盘空间.
* 目录项 (dentry) 记录文件的名字、索引节点指针以及与其他目录项的层级关联关系. 多个目录项关联起来, 就会形成目录结构.
    * 与索引节点不同, 目录项是由内核维护的一个数据结构, 不存放于磁盘, 而是缓存在内存.
    ```c
    // Linux内核 5.x
    struct dentry {
        atomic_t d_count;			/* 引用计数 */
        unsigned int d_flags;			/* 相关标志 */
        spinlock_t d_lock;			/* 用于同步的自旋锁 */
        const struct dentry_operations *d_op;	/* dentry操作函数指针 */
        struct super_block *d_sb;		/* 指向超级块对象的指针 */
        unsigned long d_time;			/* dentry失效的时间 */
        void *d_fsdata;			/* 用于指向文件系统特定数据的指针 */
        
        struct inode *d_inode;			/* 指向对应inode的指针 */
        struct hlist_bl_node d_hash;		/* 哈希链表节点 */
        struct dentry *d_parent;		/* 指向父dentry的指针 */
        struct qstr d_name;			/* 文件名 */
        struct list_head d_lru;			/* 用于LRU链表的节点 */
        struct list_head d_subdirs;		/* 子目录链表头 */
        struct list_head d_u.d_child;		/* 用于兄弟dentry链表的节点 */
        struct dentry *d_mounted;		/* 指向安装点的指针 */
        unsigned long d_iname[DNAME_INLINE_LEN_MIN]; /* 短文件名直接存储空间 */
    };
    ```
另外磁盘进行格式化的时候, 会被分成三个存储区域: 1) 超级块; 2) 索引节点区; 3) 数据块区.
* 超级块: 存储*文件系统*的详细信息. 如块个数、块大小、空闲块等等.
* 索引节点区: 用来存储索引节点.
* 数据块区: 用来存储文件或目录数据.

## 查看一个 文件/目录 的 inode
```bash 
# ls -i 
# OR
stat <file-name>
```

## 硬链接 (hard links)
* 我们通过文件名 (元数据) --> inode --> 文件内容 (数据块)

需求: 系统中可能有同一个运行库的多个版本 (libc-2.27.so, libc-2.26.so, ...), 还需要一个 "当前版本的 libc", 程序需要链接 "libc.so.6", 能否避免文件的一份拷贝？
* 多个名字解析到同一个 inode 上 --> 硬链接

### [Lab - hard links]
1. 为本文件创建两个硬链接 `hardlink.one, hardlink.two` (节省服务器的磁盘空间):
    ```bash
    ln ./hard_symbol_link.md hardlink.one
    ln ./hard_symbol_link.md hardlink.two
    ls -li 
    # inode             links
    # 96885007 -rw-rw-r-- 3 uni01 uni01 2681 May 17 13:41 hardlink.one
    # 96885007 -rw-rw-r-- 3 uni01 uni01 2681 May 17 13:41 hardlink.two
    # 96885007 -rw-rw-r-- 3 uni01 uni01 2681 May 17 13:41 hard_symbol_link.md
    ```
2. 删除掉某一个硬链接文件并不会影响 inode 号相同的其他文件. 只有当 inode 记录的链接数为 0 时, 数据块才会被真正删除 (防止文件被"误删"):
    ```bash
    rm hardlink.one
    ls -li 
    ```
**限制:**
1. 不能链接目录
2. 不能跨越不同的文件系统 (因为 inode 在自己的文件系统之外没有任何意义).

## 符号链接 (symlinks)
软链接： 在文件里存储一个 "跳转提示"
* 为允许跨文件系统建立链接 --> 实现符号链接
* 每个 symlink 都有自己的 inode

### [Lab - symlinks]
1. 新建文件 `sample.txt`, 并为其创建一个软链接 `softlink.txt`:
    ```bash
    echo LAB-SOFTLINK > sample.txt
    ln -s sample.txt softlink.txt
    ls -li
    ```
    发现 
    (1) 软链接和其所指向的文件具有不同的 inode, 其数据块存储的内容为其所指向文件的路径.
    (2) 创建软链接, 其指向文件的硬链接次数不会增加

2. 删除原文件
    ```bash
    rm -rf sample.txt
    # 删除软链接并不影响原文件, 但删掉原文件, 软链接会成为一个"死"链接 (软链接可以指向不存在的文件或目录)
    ```
软链接可以跨文件系统创建, 可以指向文件, 也可以指向目录.