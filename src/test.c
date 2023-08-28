#include <stdio.h>

// shortとかの変数は勝手にひっくり返してくれる
// メモリ中に並んでいる値はキャストするとリトルエンディアンとして処理される
int main() {
    unsigned short val = 1;
    unsigned char *p = (unsigned char*)&val;
    unsigned char ary[] = {0, 1};
    unsigned short val2;
    printf("p[0] = %x\n", p[0]); // 1 勝手にひっくり返す
    printf("p[1] = %x\n", p[1]); // 0  
    
    val2 = *(short*)ary;
    printf("val2 = %x\n", val2); //-->  メモリ中に00 01の順序で並んでいるが，キャストすると0x0100となる

    return 0;
}