%{
#include<stdio.h>
#include "y.tab.h"
%}

%%
    /* 0から9という数字が一つ以上繰り返されたもの */
[0-9]+ {yylval = atoi(yytext); return NUMBER;}
    /* 空白を読み飛ばすため*/
[ \t]+
    /* 入力が確定したことを知るため */
\n return 0;
    /* 上記以外の任意の文字 */
. return yytext[0];
%%
