#include <stdio.h>
#include <stdlib.h>
#include <math.h>

static int** memory;
static int* cells;
static int max_val;
static int segment_size;
static int size;

#define G(r, c) [((size*r)+c)]
#define pf printf

void ib(char* fname){FILE*f=fopen(fname,"r");if(fscanf(f, "%d",&size)!=1)goto id;max_val=size;segment_size=0;if(cells!=NULL)free(cells);cells=(int*)malloc(sizeof(int)*(size*size));for(int i=0;;i++){if(i==(size*size))break;if(fscanf(f,"%d",&cells[i])!=1){break;}if(i==0)continue;if(size/i==i&&size%i==0)segment_size=i;}id:fclose(f);return;}void pl(int o){pf(" ");for(int j=1;j<o;j++){pf("-");}pf("\n");}void pb(){int m=floor(log10(size))+1;int o=(size*m)+(2*segment_size)+3+(size-1);for(int i=0;i<size;i++){if(i%segment_size==0)pl(o);for(int j=0;j<size;j++){if(j%segment_size==0){pf(" |");}int v=cells G(i,j);int l=m-1;if(v!=0)l=(m-floor(log10(v)))-1;for(int k=0;k<l;k++){pf(" ");}pf(" %d", v);}pf(" |\n");}pl(o);}int check(int v,int r,int c){for(int i=0;i<size;i++){if(cells G(r,i)==v)return 0;if(cells G(i,c)==v)return 0;}int y=(r/segment_size)*segment_size;int f=(c/segment_size)*segment_size;for(int p=y;p<y+segment_size;p++){for(int g=f;g<f+segment_size;g++){if(g==c&&p==r)continue;if(cells G(p,g)==v)return 0;}}return 1;}int n(int r,int c){if(r>=size)return 1;if(cells G(r,c)!=0){if(c+1>=size)return n(r+1,0);else return n(r,c+1);}for(int v=1;v<=max_val;v++){if(check(v,r,c)!=0){cells G(r, c) = v;if(c+1>=size)if(n(r+1,0)== 0) cells G(r,c)=0;else break;else if(n(r,c+1)==0) cells G(r,c)=0;else break;}}return cells G(r,c)!=0;}

int mem_one(int r, int c) {
    int ret = 0;

    for (int i=0;i<size;i++) {
        ret += memory G(r, c) [i];
    }

    return ret;
}

int pop(int r, int c) {
    for(int i=0;i<size;i++){
        int t = memory G(r, c)[i];
        if (t) return t;
    }
    return -1;
}

void clear(int r, int c) {
    for (int i=0;i<size;i++) {
        memory G(r, c)[i] = 0;
    }
}

void mem_remove(int v, int r, int c, int ri, int re, int ci, int ce) {
    for (int i=0;i<size;i++) {
        memory G(r, i)[v] = 0;
        memory G(i, c)[v] = 0;
    }

    for (int i=ri;i<re;i++) {
        for(int j=ci;j<ce;j++) {
            memory G(i, j)[v] = 0;
        }
    }
}

int b() {

    return n(0, 0);
}

int main(int c,char** a){for(int i=1;i<c;i++){ib(a[i]);};b();pb();return 0;}
