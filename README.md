# 工具箱

一个使用Rust语言和Slint UI框架实现的Mac上的小工具箱，但这些工具基本上都是伪需求，目的是熟悉Rust语言，不可用于生产环境。

## 关于Rust

先说明本人实力：练气期二层。

我仍然觉得Python代码是最漂亮的，只不过它的GIL锁让我在之前的项目中陷入困境，如果PEP 703落地我还会重新把Python作为个人的主力语言。至于为什么在其他语言中选择Rust接替Python：我愿意让渡一些灵活和完全控制的自由来换取程序的稳定。当然，这主要是因为我没有把握完全利用这些自由和灵活性。在一些对效率极度敏感的场景中，我认为用指针指来指去仍然是最好的（对于元婴期以上大佬），毕竟就算是0成本抽象也是对机器而言，而不是对人。

## 关于Slint

之前我已经使用iced完成了toolbox的部分开发，但是，它的字体支持让我觉得即使熟悉了以后用在工作上的可能性也不大，所以还是选择了Slint。

## 注意

请勿提交代码。这是个人熟悉Rust和Slint的练习项目。