// 分析软链，类似windows 的快捷方式
use std::fs

let path= fs::read_link("a.txt");

println("{}",path)