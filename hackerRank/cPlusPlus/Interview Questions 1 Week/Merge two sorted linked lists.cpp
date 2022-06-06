SinglyLinkedListNode* mergeLists(SinglyLinkedListNode* head1, SinglyLinkedListNode* head2) {
    SinglyLinkedListNode * head;
    SinglyLinkedListNode * result;
    //Check with will be head
    if(head1->data<head2->data){
        head = head1;
        head1 = head1->next;
    }
    else{
        head = head2;
        head2 = head2->next;
    }
    result = head;
    while(head1!=NULL||head2!=NULL){
        int h1 = 2000;
        int h2 = 2000;
        if(head1!=NULL){
            h1 = head1->data;
        }
        if(head2!=NULL){
            h2 = head2->data;
        }
        if(h1<=h2&&h1!=2000){
            head->next = head1;            
            head1 = head1->next;
            head = head->next;
        }
        else if(h2<=h1&&h2!=2000){
            head->next = head2;            
            head2 = head2->next;
            head = head->next;
        }
    }
    
    return  result;
}
