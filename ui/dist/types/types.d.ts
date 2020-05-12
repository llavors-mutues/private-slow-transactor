import { Agent } from 'holochain-profiles';
import { ApolloClient } from 'apollo-boost';
export declare const allAgentsAllowed: GetAllowedCreditors;
export declare type GetAllowedCreditors = (client: ApolloClient<any>) => Promise<Agent[]>;
export interface Transaction {
    id: string;
    debtor: Agent;
    creditor: Agent;
    amount: number;
    timestamp: number;
}
export interface CounterpartySnapshot {
    valid: boolean;
    executable: boolean;
    invalidReason: string;
    balance: number;
    lastHeaderId: string;
}
export interface Counterparty {
    online: boolean;
    consented: boolean;
    snapshot?: CounterpartySnapshot;
}
export interface Offer {
    id: string;
    transaction: Transaction;
    state: string;
    counterparty: Counterparty;
}
