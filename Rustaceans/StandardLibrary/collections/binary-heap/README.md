# 二叉堆
Credit to <https://labuladong.github.io/algo/di-yi-zhan-da78c/shou-ba-sh-daeca/er-cha-dui-1a386/>

Binary Heap 
* 主要操作就两个: **下沉**和**上浮**, 用以维护二叉堆的性质.
* 主要应用有两个: 首先是一种排序方法「堆排序」; 第二是一种很有用的数据结构「优先级队列」.
* 分为最大堆和最小堆: 最大堆的每个节点都大于等于 (>=) 它的两个子节点; 最小堆的每个节点都小于等于 (<=) 它的子节点.

二叉堆在逻辑上其实是一种特殊的二叉树 (完全二叉树), 只不过存储在数组里, 操作数组的索引. (一般的链表二叉树, 我们操作节点的指针)
[画个图](https://labuladong.github.io/algo/di-yi-zhan-da78c/shou-ba-sh-daeca/er-cha-dui-1a386/)你立即就能理解了
数组的第一个索引 0 空着不用, 所以会有公式
```text
父节点idx = 子节点idx / 2
左孩子节点idx = 父节点idx * 2
右孩子节点idx = 父节点idx * 2 + 1
```

以最大堆为例, 每个节点都 >= 两个子节点, 在插入元素和删除元素时, 难免破坏堆的性质. 这就需要通过这两个操作来恢复堆的性质了:
* 如果某个节点 A 比它的子节点 (中的一个) 小, 那么 A 就不配做父节点, 应该下去, 下面那个更大的节点上来做父节点, 这就是对 A 进行下沉. (使得第k个元素不断和两个子节点进行比较, 下沉到合适的位置, 平衡优先级. 常常和delMax(删除最大堆的堆顶)使用, 首先将堆首位对调, 删除末位子节点后将首元素下沉到合适位置)
* 如果某个节点 A 比它的父节点大, 那么 A 不应该做子节点, 应该把父节点换下来, 自己去做父节点, 这就是对 A 的上浮. (使得第 k 个元素不断和父节点比较、上浮, 进而平衡树的优先级. 常常和insert使用, 即将元素插入末位子节点, 然后进行上浮)


# 优先级队列


