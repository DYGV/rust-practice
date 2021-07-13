#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// 1つのトークン(数字を含む)は10文字(桁)までとする
#define MAX_TOKEN_DIGIT 10
// 演算子と被演算子の最大個数
#define MAX_TOKEN_NUM 20

struct Token;
struct Statement;
struct Stack;
enum Operator;
struct Token *insert(struct Token *parent, struct Token *child);
struct Token *new_node();
int is_operand(char reading_char);
int is_operator(char reading_char);
struct Token *make_tree(char *input);
void to_postfix(struct Token *tokens);
void push(struct Stack *stack, int val);
int pop(struct Stack *stack);
void calculate(struct Token *tokens, struct Stack *stack);
int calculator(char *input);


// 木構造をつくるための構造体(ノードそのもの)
struct Token{
    // 1つのトークンをMAX_TOKEN_DIGITまでとする
    char data[MAX_TOKEN_DIGIT+1];
    // 被演算子かどうか
    int is_operand;
    // 子
    struct Token *left, *right;
};

// 式を被演算子、演算子に分けた構造体
struct Statement{
    int lhs;
    int rhs;
    char operator;
};

struct Stack{
    int stack[MAX_TOKEN_NUM];
    int sp;
};

// 四則演算の演算子定義
enum Operator{
    PLUS = '+',
    MINUS = '-',
    MULTI = '*',
    DIV = '/'
};

// 演算子順序の比較をする関数
// inputがcurrentより計算順序の優先度が高いかどうか
int weight_cmp(char current, char input){
    return ((current == PLUS || current == MINUS) && (input == MULTI || input == DIV));
}

// ノードの新しい領域確保する関数
struct Token *new_node(){
    struct Token *new_node =  (struct Token *)malloc(sizeof(struct Token));
    if(!new_node){
        exit(1);
    }
    new_node->left = NULL;
    new_node->right = NULL;
    strcpy(new_node->data, "\0");
    return new_node;
}

// 新たなノードを挿入する関数
// オペレータが親、オペランド2つが子から成る部分木を生成、つなげていく
struct Token *insert(struct Token *parent, struct Token *child){
    // 被演算子だったとき
    if(child->is_operand){
        // 左が空なら
        if(!parent->left){
            parent->left = child;
        // 右が空なら
        }else if(!parent->right){
            parent->right = child;
        // 両方とも空いていないなら
        }else{
            // 再帰的に右端へ向かって移動していく
            insert(parent->right, child);
        }
        return parent;
    }

    //////////// 以降の処理は入力が演算子///////////////
    if((!parent->left || !parent->right)){
        // 入力の一番最初がマイナス(負数) (例: -aなら -1*aにする)
        if(*child->data == MINUS){
            *parent->data = MULTI;
            struct Token *left = new_node();
            parent->left = left;
            strcpy(parent->left->data, "-1");
            parent->left->is_operand = 1;
            return parent;
        }
        strcpy(parent->data, child->data);
        return parent;
    }

    // 左右がNULLでない(埋まっている)状態で
    // 入力記号の方が優先度が高い
    if(weight_cmp(*parent->data, *child->data)){
        struct Token *bak = new_node();
        memcpy(bak, parent->right, sizeof(struct Token)); // parentの右の子を退避
        memcpy(parent->right, child, sizeof(struct Token)); // 現在の右の子のメモリ上にchildをコピー
        parent->right->left = bak; // 退避しておいたノード(bak)をparent->right->leftに入れる
        return parent;
    }

    // 左右がNULLでない(埋まっている)状態で
    // 入力記号の優先度が同じもしくは低い
    struct Token *bak = new_node();
    memcpy(bak, parent, sizeof(struct Token)); // parentを退避
    memcpy(parent, child, sizeof(struct Token)); // 現在のparentのメモリ上childをコピー
    parent->left = bak; // 退避しておいたノード(bak)をparent->leftに入れる
    return parent; 
}

// 入力文字がアスキーコードの0-9にあるかを調べる関数
int is_operand(char reading_char){
    return (reading_char >='0' && reading_char<='9');
}

// 入力文字が演算子であるかを調べる関数
int is_operator(char reading_char){
    return (reading_char == PLUS ||  reading_char == MINUS 
            || reading_char == MULTI || reading_char == DIV);
}

// 木構造をつくるための関数
struct Token* make_tree(char *input){
    struct Token *current_node = new_node();
    // 先頭位置の記録用のポインタ
    struct Token *root = current_node;
    while(*input){
        char *operand = (char *)malloc(sizeof(char) * MAX_TOKEN_DIGIT + 1);
        char *operator = (char *)malloc(sizeof(char) * MAX_TOKEN_DIGIT + 1);
        if(!operand || !operator){
            exit(1);
        }
        // 先頭位置の記録用のポインタ
        char *operand_head = operand;
        char *operator_head = operator; 
        // スペースや改行文字など関係ないものはスキップする
        while(*input == ' ' || *input == '=' || *input=='\r'|| *input=='\n'){
            input++;
        }
        while(is_operand(*input)){
            *operand++ = *input++;
        }
        while(is_operator(*input)){
            *operator++ = *input++;
        }
        // 被演算子があるなら
        if(strlen(operand_head)){
            struct Token *child_operand = new_node();
            strcpy(child_operand->data, operand_head);
            child_operand->is_operand = 1;
            current_node = insert(current_node, child_operand);
        }
        // 演算子があるなら
        if(strlen(operator_head)){
            struct Token *child_operator = new_node();
            strcpy(child_operator->data, operator_head);
            child_operator->is_operand = 0;
            current_node = insert(current_node, child_operator);
        }
    }
    return root;
}

// 木を帰りがけ順で出力する関数(後置記法)
void to_postfix(struct Token *tokens){
    if(!tokens){
        return;
    }
    to_postfix(tokens->left);
    to_postfix(tokens->right);
    printf("%s ", tokens->data);
}

// 木を通りがけ順で出力する関数(中間記法)
void to_infix(const struct Token *tokens){
    if(!tokens){
        return;
    }
    to_infix(tokens->left);
    printf("%s ", tokens->data);
    to_infix(tokens->right);
}

// Statementをもとに計算する関数
int calc(struct Statement *s){
    int result;
    switch(s->operator){
        case PLUS:
            result = s->lhs + s->rhs;
            break;
        case MINUS:
            result = s->lhs - s->rhs;
            break;
        case MULTI:
            result = s->lhs * s->rhs;
            break;
        case DIV:
            result = s->lhs / s->rhs;
            break;
        default:
           // 演算子でないときは終了
           exit(1);
    }
    return result;
}

void push(struct Stack *stack, int value){
    printf("スタック%d番目に%dをpush\n", stack->sp, value);
    stack->stack[stack->sp++] = value;
}

int pop(struct Stack *stack){
    stack->sp--;
    if(stack->sp < 0){
        stack->sp = 0;
    }
    printf("スタック%d番目の%dをpop\n", stack->sp, stack->stack[stack->sp]);
    return stack->stack[stack->sp];
}

// 帰りがけ順で式を計算する関数(後置記法)
void calculate(struct Token *tokens, struct Stack *stack){
    if(!tokens->left && !tokens->right){
        return;
    }
    calculate(tokens->left, stack);
    calculate(tokens->right, stack);
    struct Statement *statement = (struct Statement *)malloc(sizeof(struct Statement));
    if(!statement){
        exit(1);
    }
    // tokensの左右(サブツリーの子)が被演算子ならそれを使う。演算子ならスタックから取ってくる
    statement->lhs = tokens->left->is_operand ? atoi(tokens->left->data) : pop(stack);
    statement->rhs = tokens->right->is_operand ? atoi(tokens->right->data) : pop(stack);
    statement->operator = (char)*tokens->data;
    push(stack, calc(statement)); // 演算結果をスタックに積む
    free(statement);
}

int calculator(char *input){
    // 構文木を作る
    struct Token *tokens = make_tree(input);
    struct Stack *stack = (struct Stack*)calloc(1, sizeof(struct Stack)); 
    printf("\n中間記法: ");
    to_infix(tokens);
    printf("\n");
    printf("後置記法: ");
    to_postfix(tokens);
    printf("\n\n");
    // 計算する
    calculate(tokens, stack);
    // tokensが必要なくなったので開放
    free(tokens);
    // 計算結果をスタックから下ろす
    int result = pop(stack);
    // stackが必要なくなったので開放
    free(stack);
    // 計算結果を返す
    return result;
}

int main(){
    char input[50];
    while(fgets(input, 50, stdin)){
        printf("式: %s", input);
        int answer = calculator(input);
        printf("\n答え : %d\n", answer);
        printf("---------------------------------------------\n");
     }
}
