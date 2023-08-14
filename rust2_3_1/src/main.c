//
// Created by hentai8-desktop on 23-8-14.
//

// 内存溢出测试
// 悬空指针
// error
int *foo() {
    int a;
    a = 100;
    char *c = "xyz";
    return &a;
}

int main() {
    int *p = foo();
    printf("%d\n", *p);
    return 0;
}