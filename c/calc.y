//lex calc.l && yacc -d calc.y && gcc y.tab.c lex.yy.c -ll -o calc
%{
#include <stdio.h>
#include <string.h>

extern int yylex();
extern int yyparse();

void yyerror(const char *str)
{
    fprintf(stderr,"error: %s\n",str);
}

int main()
{
    printf("入力: ");
    // 構文解析をする
    // yyparse()の中でyylex()を呼び出して字句解析も行っている
    yyparse();
}
%}

// トークンの定義
%token NUMBER
// 開始記号をSとする
%start S
%%

/*
  BNF記法と同じような規則を書いていく。
  A、B、Cの順に演算の優先度が高くなっていく。
  ⟨NUMBER⟩ := [0-9]
  ⟨S⟩ := ⟨A⟩
  ⟨A⟩ := ⟨A⟩  '+'  ⟨B⟩ | ⟨A⟩  '-'  ⟨B⟩ | ⟨B⟩
  ⟨B⟩ := ⟨B⟩  '*'  ⟨C⟩ | ⟨B⟩  '/'  ⟨C⟩ | ⟨C⟩
  ⟨C⟩ := '(' ⟨B⟩ ')' | ⟨NUMBER⟩
*/

S : A {printf("答えは%d\n", $1);};

// Aでの評価: 加算、減算
A : A '+' B {$$ = $1 + $3; printf("%d + %d = %d\n",$1, $3, $$);}
  | A '-' B {$$ = $1 - $3; printf("%d - %d = %d\n",$1, $3, $$);}
  | B
;

// Bでの評価: 乗算、除算
B : B '*' C {$$ = $1 * $3; printf("%d * %d = %d\n",$1, $3, $$);}
  | B '/' C {$$ = $1 / $3; printf("%d / %d = %d\n",$1, $3, $$);}
  | C
;

// Cでの評価: (式)
C : '(' A ')' {$$ = $2;}
  | NUMBER
;
%%
