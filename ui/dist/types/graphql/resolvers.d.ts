export declare const resolvers: {
    Transaction: {
        creditor(parent: any): {
            id: any;
        };
        debtor(parent: any): {
            id: any;
        };
    };
    Offer: {
        counterpartySnapshot(parent: any, _: any, { container }: {
            container: any;
        }): Promise<any>;
    };
    CounterpartySnapshot: {
        lastHeaderId(parent: any): any;
    };
    Query: {
        offer(_: any, { transactionId }: {
            transactionId: any;
        }, { container }: {
            container: any;
        }): Promise<{
            id: any;
            transaction: any;
            state: any;
        }>;
    };
    Me: {
        transactions(_: any, __: any, { container }: {
            container: any;
        }): Promise<any>;
        offers(_: any, __: any, { container }: {
            container: any;
        }): Promise<any>;
        balance(_: any, __: any, { container }: {
            container: any;
        }): Promise<any>;
    };
    Mutation: {
        createOffer(_: any, { creditorId, amount }: {
            creditorId: any;
            amount: any;
        }, { container }: {
            container: any;
        }): Promise<any>;
        acceptOffer(_: any, { transactionId, approvedHeaderId }: {
            transactionId: any;
            approvedHeaderId: any;
        }, { container }: {
            container: any;
        }): Promise<any>;
        cancelOffer(_: any, { transactionId }: {
            transactionId: any;
        }, { container }: {
            container: any;
        }): Promise<any>;
    };
};
