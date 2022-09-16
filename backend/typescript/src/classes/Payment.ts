import { HasFormatter } from "../interfaces/HasFormatter.js";

export class Payment implements HasFormatter {
  constructor(
    readonly precipient: string,
    private details: string,
    public amount: number
  ) {}

  format() {
    return `${this.precipient} is owed ${this.amount} for ${this.details}`;
  }
}
