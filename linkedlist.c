#include "linkedlist.h"

#include "types.h"

#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>


LList* ll_create(){
  LList* list = malloc(sizeof(LList));
  assert(list != NULL);
  list->head = NULL;
  list->length = 0;
  return list;
}

void ll_check(LList* list) {
  // check existence
  assert(list != NULL);

  //check length is positive
  assert(list->length >= 0);

  // Check length is correct:
  if (list->length > 0) {
    //Check conditions for non-zero length:
    // aka, check if length-1th child is a leaf
    assert(list->head != NULL);

    LNode* curr = list->head;

    for (int i = 1; i< list->length; i++) {
      assert(curr != NULL);
      curr = curr->next;
    }
    assert(curr->next == NULL);
  } else {
    // check conditions for zero-length:
    // aka check if head is equal to nulist pointer
    assert(list->head == NULL);
  }
}

void ll_free_recur(LList* list) {
  //assert existence
  assert(list != NULL);
  if (list->length > 0) {

    //INVARIANT: curr is non-NULL
    LNode* curr = list->head;
    LNode* child = curr->next;

    while (child != NULL) {
      ln_free_recur(curr);
      curr = child;
      child = child->next;
    }

    // INVARIANT: child == NULL
    ln_free_recur(curr);
  }
  ll_free(list);
}
void ll_free(LList* list) {
  //assert existence
  assert(list != NULL);

  //Free list
  free(list);
}

void ll_asprint(LList* list, char** ret) {
  if (list->length == 0) {
    asprintf(ret, "()");
    return;
  }
  //TODO: error check asprintf

  // populate strings
  char** strs = malloc(sizeof(char*) * list->length);
  if (strs != NULL) {
    LNode* curr = list->head;

    int tchars = 0;
    tchars += 1; // "("

    for (int i = 0; i < list->length; i++) {
      ln_asprint(curr, &strs[i]);

      tchars += strlen(strs[i]);
      tchars += 1;

      curr = curr->next;
    }

    /* for (int i = 0; i < list->length; i++) { */
    /*   printf("\nstrs[%d]: %s\n", i, strs[i]); */
    /* } */

    //concat strings
    char* output = malloc(tchars + 1);
    if (output!= NULL) {
      int pos = 0;

      output[0] = '(';
      pos += 1;

      int i;
      for (i = 0; i < (list->length - 1); i++) {
        memcpy(&(output[pos]), strs[i], strlen(strs[i]));
        pos += strlen(strs[i]);
        free(strs[i]); //done with it, can free.

        output[pos] = ' ';
        pos +=1;
      }
      memcpy(&(output[pos]), strs[i], strlen(strs[i]));
      pos += strlen(strs[i]);
      free(strs[i]); //done with it, can free.

      output[pos] = ')';
      pos += 1;

      output[pos] = '\0';
      pos += 1;

      assert(pos == tchars+1);

      *ret = output;
    } else {
      printf("ERROR: Ran out of memory!\n");
      for (int i = 0; i < list->length; i ++){
        free(strs[i]);
      }
      ret = NULL;
    }

    free(strs);
  } else {
    printf("ERROR: Ran out of memory!\n");
    ret = NULL;
  }
}

void ll_print(LList* list) {
  char* str;
  ll_asprint(list, &str);
  if (str != NULL) {
    printf("%s", str);
    free(str);
  }else {
    printf("Error! out of memory!\n");
  }
}




LNode* ll_get(LList* list, int index) {
  assert(index >= 0 && index < list->length);

  LNode* curr = list->head;
  for (int i = 0; i < index; i++) {
    curr = curr->next;
  }
  return curr;
}

// MUTATORS

// private
void ll_push(LList* list, LNode* node) {
  assert(node != NULL);

  assert(node->next == NULL); //assert node is atomic

  if (list->length == 0) {
    list->head = node;
    list->length = 1;
  }else {
    LNode* tail = ll_get(list, list->length - 1);

    tail->next = node;
    list->length += 1;
  }
}


// remove from end, returning pointer to element
LNode* ll_pop(LList* list) {
  assert(list->length > 0);

  if (list->length == 1) {
    LNode* output = list->head;
    list->head = NULL;
    list->length = 0;
    return output;
  }


  //INVARIANT: list->length >= 2

  LNode* parent = ll_get(list, list->length - 2);
  LNode* output = parent->next;

  parent->next = NULL;
  list->length -= 1;

  return output;
}

LNode* ll_shift(LList* list) {
  assert(list->length > 0);

  if (list->length == 1) {
    LNode* output = list->head;
    list->head = NULL;
    list->length = 0;
    return output;
  }

  //INVARIANT: list->length >= 2
  LNode* output = list->head;
  list->head = output->next;
  list->length -= 1;
  return output;
}

void ll_unshift(LList* list, LNode* node) {
  assert(node->next == NULL); //assert node has no connections
  if (list->length == 0) {
    list->head = node;
    list->length = 1;
  } else {
    LNode* newhead = node;
    newhead->next = list->head;

    list->head = newhead;
    list->length += 1;
  }
}
