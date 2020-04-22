export interface Agent {
  id: string;
}

export interface Transaction {
  id: string;

  debtor: Agent;
  creditor: Agent;
  amount: Number;
  timestamp: Number;
}
