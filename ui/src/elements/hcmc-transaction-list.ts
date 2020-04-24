import { LitElement, property, html } from 'lit-element';
import { ApolloClient } from 'apollo-boost';

import { ApolloClientModule } from '@uprtcl/graphql';
import { moduleConnect } from '@uprtcl/micro-orchestrator';

import '@material/mwc-top-app-bar';
import '@material/mwc-list';
import '@authentic/mwc-circular-progress';

import { GET_MY_TRANSACTIONS } from '../graphql/queries';
import { Transaction } from '../types';

export class TransactionList extends moduleConnect(LitElement) {
  @property({ type: Object, attribute: false })
  transactions!: Array<Transaction>;

  async firstUpdated() {
    const client: ApolloClient<any> = this.request(
      ApolloClientModule.bindings.Client
    );
    const result = await client.query({
      query: GET_MY_TRANSACTIONS,
    });

    this.transactions = result.data.myTransactions;
  }

  render() {
    if (!this.transactions)
      return html` <mwc-circular-progress></mwc-circular-progress> `;

    return html`
      <mwc-list>
        ${this.transactions.map(
          (transaction) => html`
            <mwc-list-item>
              ${transaction.debtor.id} => ${transaction.creditor.id},
              ${transaction.amount}
            </mwc-list-item>
            <mwc-list-divider></mwc-list-divider>
          `
        )}
      </mwc-list>
    `;
  }
}
