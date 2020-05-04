import { LitElement, property, html } from 'lit-element';
import { ApolloClient } from 'apollo-boost';

import { ApolloClientModule } from '@uprtcl/graphql';
import { moduleConnect } from '@uprtcl/micro-orchestrator';

import '@material/mwc-top-app-bar';
import '@material/mwc-list';
import '@authentic/mwc-circular-progress';

import { GET_MY_TRANSACTIONS } from '../graphql/queries';
import { Transaction } from '../types';
import { Agent } from 'holochain-profiles';
import { sharedStyles } from './sharedStyles';

export class MCTransactionList extends moduleConnect(LitElement) {
  @property({ type: String })
  myAgentId!: string;

  @property({ type: Object, attribute: false })
  transactions!: Array<Transaction>;

  static get styles() {
    return sharedStyles;
  }

  async firstUpdated() {
    const client: ApolloClient<any> = this.request(
      ApolloClientModule.bindings.Client
    );
    const result = await client.query({
      query: GET_MY_TRANSACTIONS,
    });

    this.myAgentId = result.data.me.id;
    this.transactions = result.data.myTransactions;
  }

  isOutgoing(transaction: Transaction) {
    return transaction.debtor.id === this.myAgentId;
  }

  getCounterparty(transaction: Transaction): Agent {
    return transaction.creditor.id === this.myAgentId
      ? transaction.debtor
      : transaction.creditor;
  }

  render() {
    return html`<div class="column center-content">
      ${this.renderContent()}
    </div>`;
  }

  renderContent() {
    if (!this.transactions)
      return html`
        <div class="padding center-content">
          <mwc-circular-progress></mwc-circular-progress>
        </div>
      `;

    if (this.transactions.length === 0)
      return html`<div class="padding">
        <span>You have no transactions in your history</span>
      </div>`;

    return html`
      <mwc-list style="width: 100%;">
        ${this.transactions.map(
          (transaction) => html`
            <mwc-list-item twoline noninteractive>
              <span>
                ${this.isOutgoing(transaction) ? 'To ' : 'From '}
                @${this.getCounterparty(transaction).username}:
                ${`${this.isOutgoing(transaction) ? '-' : '+'}${
                  transaction.amount
                }`}
                credits
              </span>
              <span slot="secondary"
                >${this.getCounterparty(transaction).id}) on
                ${new Date(
                  transaction.timestamp * 1000
                ).toLocaleDateString()}</span
              >
            </mwc-list-item>
            <mwc-list-divider></mwc-list-divider>
          `
        )}
      </mwc-list>
    `;
  }
}
