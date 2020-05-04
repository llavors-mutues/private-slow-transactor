import { Agent } from 'holochain-profiles';
import { ApolloClient } from 'apollo-boost';
export declare const allAgentsAllowed: GetAllowedCreditors;
export declare type GetAllowedCreditors = (client: ApolloClient<any>) => Promise<Agent[]>;
export interface Transaction {
    id: string;
    debtor: Agent;
    creditor: Agent;
    amount: Number;
    timestamp: Number;
}
export interface CounterpartySnapshot {
    valid: boolean;
    executable: boolean;
    balance: number;
    lastHeaderId: string;
}
export interface Offer {
    id: string;
    transaction: Transaction;
    state: string;
    counterpartySnapshot: CounterpartySnapshot;
}
