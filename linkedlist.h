#include "types.h"

#ifndef LINKEDLIST_C
#define LINKEDLIST_C



struct LList_ {
  LNode* head;
  int length;
};

LList* ll_create();

void ll_check(LList* list);
void ll_free(LList* list); //doesn't touch element memory
void ll_free_recur(LList* list); //frees each element's memory, as well.

void ll_asprint(LList* list, char** ret); //Must check ret != NULL
void ll_print(LList* list);

LNode* ll_get(LList* list, int index);

void ll_push(LList* list, LNode* node);
LNode* ll_pop(LList* list);
LNode* ll_shift(LList* list);
void ll_unshift(LList* list, LNode* node);

#endif
