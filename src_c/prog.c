#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef uint64_t u64;

static u64 x, *C, H, S, *M;

#define G(i, j) [((S*i)+j)]
#define pf printf
#define rt return

void ib(char*n){FILE*f=fopen(n,"r");if(fscanf(f, "%ld",&S)!=1)goto id;x=S;H=0;if(C!=NULL)free(C);C=(u64*)malloc(sizeof(u64)*(S*S));for(u64 i=0;;i++){if(i==(S*S))break;if(fscanf(f,"%ld",&C[i])!=1)break;if(i==0)continue;if(S/i==i&&S%i==0)H=i;}id:fclose(f);rt;}void pl(u64 o){pf(" ");for(u64 j=1;j<o;j++)pf("-");pf("\n");}void pb(){u64 m=floor(log10(S))+1;u64 o=(S*m)+(2*H)+(S+2);for(u64 i=0;i<S;i++){if(i%H==0)pl(o);for(u64 j=0;j<S;j++){if(j%H==0)pf(" |");u64 v=C G(i,j);u64 l=m-1;if(v!=0)l=(m-floor(log10(v)))-1;for(u64 k=0;k<l;k++){pf(" ");}pf(" %ld", v);}pf(" |\n");}pl(o);}u64 ch(u64 v,u64 r,u64 c){for(u64 i=0;i<S;i++){if(C G(r,i)==v)rt 0;if(C G(i,c)==v)rt 0;}u64 y=(r/H)*H;u64 f=(c/H)*H;for(u64 p=y;p<y+H;p++)for(u64 g=f;g<f+H;g++){if(g==c&&p==r)continue;if(C G(p,g)==v)rt 0;}rt 1;}u64 n(u64 r,u64 c){if(r>=S)rt 1;if(C G(r,c)!=0){if(c+1>=S)rt n(r+1,0);else rt n(r,c+1);}for(u64 v=1;v<=x;v++){if(ch(v,r,c)!=0){C G(r,c)=v;if(c+1>=S)if(n(r+1,0)== 0) C G(r,c)=0;else break;else if(n(r,c+1)==0) C G(r,c)=0;else break;}}rt C G(r,c)!=0;}void F(u64 v,u64 r,u64 c,u64 rs,u64 re,u64 zs,u64 cz) {u64 k=~(1<<v);for (int i=0;i<S;i++){M G(r, i)&=k;M G(i,c)&=k;}for(int i=rs;i<re;i++)for(int j=zs;j<cz;j++){M G(i,j)&=k;}}int Y(u64 v,u64 r,u64 c,u64 rs,u64 re,u64 zs,u64 cz){if((M G(r,c)&(1<<v))==0)rt 0;for(int i=rs;i<re;i++)for(int j=zs;j<cz;j++){if(c==j&&r==i)continue;if((M G(i,j)&(1<<v))!=0)rt 0;}C G(r,c)=v;M G(r,c)=0;F(v,r,c,rs,re,zs,cz);rt 1;}int V(u64 v,u64 r,u64 c,u64 rs,u64 re,u64 zs,u64 cz){if((M G(r,c)&(1<<v))==0)rt 0;int L=0,T=0,P=0,W=0;for(int i=rs;i<re;i++)for(int j=zs;j<cz;j++){if(r==i&&c==j)continue;if((M G(i,j)&(1<<v))!=0){if(i==r)P=1;else if(j==c)T=1;else W=1;}}if(W||(T&&P))rt 0;if(T)for(int i=0;i<S;i++){if(i>=rs&&i<re)continue;if((M G(i,c)&(1<<v))!=0){M G(i,c)&=~(1<<v);L=1;}}else if(P)for(int i=0;i<S;i++){if(i>=zs&&i<cz)continue;if((M G(r,i)&(1<<v))!=0){M G(r,i)&=~(1<<v);L=1;}}rt L;}u64 b(){if(M!=NULL)free(M);M=(u64*)malloc(sizeof(u64)*(S*S));for(u64 i=0;i<S;i++){for(u64 j=0;j<S;j++){M G(i,j)=0;if(C G(i,j)!=0)continue;for(u64 k=1;k<=x;k++)if(ch(k,i,j))M G(i,j)|=1<<k;}}int D=1;while(D){D-=1;for(u64 v=1;v<=x;v++){for(u64 r=0;r<S;r++){u64 bsr=r/H;u64 rs=bsr*H;u64 re=rs+H;for(u64 c=0;c<S;c++){if(C G(r,c)!=0)continue;u64 B=M G(r,c);u64 bsc=c/H;u64 zs=bsc*H;u64 cz=zs+H;if((B&(B-1))==0){u64 Z = 0;for(;B>1;B>>=1,Z++){}C G(r,c)=Z;M G(r,c)=0;F(Z,r,c,rs,re,zs,cz);D=1;continue;}if(Y(v,r,c,rs,re,zs,cz)){D=1;continue;}if(V(v,r,c,rs,re,zs,cz))D=1;}}}}rt n(0,0);}int main(int c,char** a){for(u64 i=1;i<c;i++)ib(a[i]);b();pb();rt 0;}
