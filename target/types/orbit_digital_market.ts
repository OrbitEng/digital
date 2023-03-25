export type OrbitDigitalMarket = {
  "version": "0.1.0",
  "name": "orbit_digital_market",
  "instructions": [
    {
      "name": "openTransactionSol",
      "docs": [
        "TRANSACTION",
        "SOL"
      ],
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "escrowAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "digitalProduct",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "BUYER SELLER",
            "BUYER"
          ]
        },
        {
          "name": "buyerMarketAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "SELLER"
          ]
        },
        {
          "name": "sellerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "EXTRANEOUS"
          ]
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "transactionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccountProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "productProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "sellerIndex",
          "type": "u8"
        },
        {
          "name": "buyerIndex",
          "type": "u8"
        },
        {
          "name": "price",
          "type": "u64"
        },
        {
          "name": "useDiscount",
          "type": "bool"
        }
      ]
    },
    {
      "name": "closeTransactionSol",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "digitalProduct",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "BUYER SELLER ACCOUNTS",
            "BUYER"
          ]
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "SELLER"
          ]
        },
        {
          "name": "sellerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerWallet",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "multisigWallet",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CPI AND EXTRANEOUS"
          ]
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccountProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "transactionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "productProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "fundEscrowSol",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "BUYER SELLER",
            "BUYER"
          ]
        },
        {
          "name": "buyerMarketAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "sellerEarlyDeclineSol",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "BUYER SELLER ACCOUNTS",
            "BUYER"
          ]
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "SELLER"
          ]
        },
        {
          "name": "sellerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerWallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CPI AND EXTRANEOUS"
          ]
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccountProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "transactionProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "openTransactionSpl",
      "docs": [
        "SPL"
      ],
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "PRODUCT"
          ]
        },
        {
          "name": "digitalProduct",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "BUYER SELLER",
            "BUYER"
          ]
        },
        {
          "name": "buyerMarketAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "SELLER"
          ]
        },
        {
          "name": "sellerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "EXTRANEOUS CPI"
          ]
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccountProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "productProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "transactionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "sellerIndex",
          "type": "u8"
        },
        {
          "name": "buyerIndex",
          "type": "u8"
        },
        {
          "name": "price",
          "type": "u64"
        },
        {
          "name": "useDiscount",
          "type": "bool"
        }
      ]
    },
    {
      "name": "closeTransactionSpl",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "digitalProduct",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "BUYER SELLER",
            "BUYER"
          ]
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "SELLER"
          ]
        },
        {
          "name": "sellerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CPI AND EXTRANEOUS"
          ]
        },
        {
          "name": "multisigAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccountProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "transactionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "productProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "fundEscrowSpl",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerMarketAccount",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "BUYER SELLER",
            "BUYER"
          ]
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CPI AND EXTRANEOUS"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "sellerEarlyDeclineSpl",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "BUYER SELLER",
            "BUYER"
          ]
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "SELLER"
          ]
        },
        {
          "name": "sellerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerWallet",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CPI AND EXTRANEOUS"
          ]
        },
        {
          "name": "marketAccountProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "transactionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "closeTransactionAccount",
      "docs": [
        "COMMON"
      ],
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposerAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "buyerAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "confirmDelivered",
      "docs": [
        "BUYER UTILS"
      ],
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerMarketAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerTransactions",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "confirmAccept",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerMarketAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerTransactions",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "denyAccept",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTransactions",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccountsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "commitInitKeys",
      "docs": [
        "SELLER UTILS"
      ],
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sellerTransactions",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerWallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "submissionKeys",
          "type": {
            "vec": "publicKey"
          }
        }
      ]
    },
    {
      "name": "commitLink",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sellerTransactions",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerWallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "link",
          "type": "string"
        }
      ]
    },
    {
      "name": "updateStatusToShipping",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sellerTransactions",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerWallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "commitSubkeys",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sellerTransactions",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerWallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "indexes",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "sellerAcceptTransaction",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sellerTransactions",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "leaveReview",
      "docs": [
        "PRODUCT",
        "REVIEW RELATED"
      ],
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "reviewedAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "REVIEW RELATED"
          ]
        },
        {
          "name": "reviewer",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "EXTRANEOUS CPI"
          ]
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "accountsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "rating",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "digitalTransaction",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "metadata",
            "type": {
              "defined": "OrbitTransactionStruct"
            }
          },
          {
            "name": "dataAddress",
            "type": "string"
          },
          {
            "name": "numKeys",
            "type": "u64"
          },
          {
            "name": "keyArr",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "finalDecision",
            "type": {
              "defined": "BuyerDecisionState"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "BuyerDecisionState",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Null"
          },
          {
            "name": "Declined"
          },
          {
            "name": "Accept"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "WrongDecisionAccount",
      "msg": "Wrong account to pay"
    },
    {
      "code": 6001,
      "name": "UndecidedTransaction",
      "msg": "Decision has not been made yet, can not close"
    },
    {
      "code": 6002,
      "name": "InvalidRateAcceptor",
      "msg": "Can not accept your own rate"
    },
    {
      "code": 6003,
      "name": "DidNotConfirmDelivery",
      "msg": "Please confirm delivery first"
    },
    {
      "code": 6004,
      "name": "WaitingForSellerData",
      "msg": "The seller did not commit keys yet"
    },
    {
      "code": 6005,
      "name": "InvalidSellerForListing",
      "msg": "The seller for the product does not match the seller given"
    },
    {
      "code": 6006,
      "name": "InvalidEscrowBump",
      "msg": "Could not compute escrow bump"
    },
    {
      "code": 6007,
      "name": "InvalidAuthBump",
      "msg": "Could not compute auth bump"
    },
    {
      "code": 6008,
      "name": "CorruptPrivateKeyFormat",
      "msg": "Could not decode private key"
    },
    {
      "code": 6009,
      "name": "IncorrectPrivateKey",
      "msg": "Private and Public keys do not match"
    },
    {
      "code": 6010,
      "name": "IndexOutOfRange",
      "msg": "Private and Public keys do not match"
    },
    {
      "code": 6011,
      "name": "CannotDiscountCommission",
      "msg": "Can not use discounts on commissions"
    },
    {
      "code": 6012,
      "name": "InvalidReflink",
      "msg": "invalid reflink passed"
    }
  ]
};

export const IDL: OrbitDigitalMarket = {
  "version": "0.1.0",
  "name": "orbit_digital_market",
  "instructions": [
    {
      "name": "openTransactionSol",
      "docs": [
        "TRANSACTION",
        "SOL"
      ],
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "escrowAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "digitalProduct",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "BUYER SELLER",
            "BUYER"
          ]
        },
        {
          "name": "buyerMarketAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "SELLER"
          ]
        },
        {
          "name": "sellerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "EXTRANEOUS"
          ]
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "transactionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccountProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "productProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "sellerIndex",
          "type": "u8"
        },
        {
          "name": "buyerIndex",
          "type": "u8"
        },
        {
          "name": "price",
          "type": "u64"
        },
        {
          "name": "useDiscount",
          "type": "bool"
        }
      ]
    },
    {
      "name": "closeTransactionSol",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "digitalProduct",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "BUYER SELLER ACCOUNTS",
            "BUYER"
          ]
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "SELLER"
          ]
        },
        {
          "name": "sellerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerWallet",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "multisigWallet",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CPI AND EXTRANEOUS"
          ]
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccountProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "transactionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "productProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "fundEscrowSol",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "BUYER SELLER",
            "BUYER"
          ]
        },
        {
          "name": "buyerMarketAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "sellerEarlyDeclineSol",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "BUYER SELLER ACCOUNTS",
            "BUYER"
          ]
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "SELLER"
          ]
        },
        {
          "name": "sellerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerWallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CPI AND EXTRANEOUS"
          ]
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccountProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "transactionProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "openTransactionSpl",
      "docs": [
        "SPL"
      ],
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "PRODUCT"
          ]
        },
        {
          "name": "digitalProduct",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "BUYER SELLER",
            "BUYER"
          ]
        },
        {
          "name": "buyerMarketAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "SELLER"
          ]
        },
        {
          "name": "sellerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "EXTRANEOUS CPI"
          ]
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccountProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "productProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "transactionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "sellerIndex",
          "type": "u8"
        },
        {
          "name": "buyerIndex",
          "type": "u8"
        },
        {
          "name": "price",
          "type": "u64"
        },
        {
          "name": "useDiscount",
          "type": "bool"
        }
      ]
    },
    {
      "name": "closeTransactionSpl",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "digitalProduct",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "BUYER SELLER",
            "BUYER"
          ]
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "SELLER"
          ]
        },
        {
          "name": "sellerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CPI AND EXTRANEOUS"
          ]
        },
        {
          "name": "multisigAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccountProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "transactionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "productProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "fundEscrowSpl",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerMarketAccount",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "BUYER SELLER",
            "BUYER"
          ]
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CPI AND EXTRANEOUS"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "sellerEarlyDeclineSpl",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "BUYER SELLER",
            "BUYER"
          ]
        },
        {
          "name": "buyerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "SELLER"
          ]
        },
        {
          "name": "sellerTransactionsLog",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerWallet",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CPI AND EXTRANEOUS"
          ]
        },
        {
          "name": "marketAccountProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "transactionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "closeTransactionAccount",
      "docs": [
        "COMMON"
      ],
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposerAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "buyerAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "confirmDelivered",
      "docs": [
        "BUYER UTILS"
      ],
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerMarketAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerTransactions",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "confirmAccept",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerMarketAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerTransactions",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "denyAccept",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTransactions",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerWallet",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccountsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "commitInitKeys",
      "docs": [
        "SELLER UTILS"
      ],
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sellerTransactions",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerWallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "submissionKeys",
          "type": {
            "vec": "publicKey"
          }
        }
      ]
    },
    {
      "name": "commitLink",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sellerTransactions",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerWallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "link",
          "type": "string"
        }
      ]
    },
    {
      "name": "updateStatusToShipping",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sellerTransactions",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerWallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "commitSubkeys",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sellerTransactions",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerWallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "indexes",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "sellerAcceptTransaction",
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerMarketAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sellerTransactions",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "leaveReview",
      "docs": [
        "PRODUCT",
        "REVIEW RELATED"
      ],
      "accounts": [
        {
          "name": "digitalTransaction",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "TX"
          ]
        },
        {
          "name": "reviewedAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "REVIEW RELATED"
          ]
        },
        {
          "name": "reviewer",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "digitalAuth",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "EXTRANEOUS CPI"
          ]
        },
        {
          "name": "digitalProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "accountsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "rating",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "digitalTransaction",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "metadata",
            "type": {
              "defined": "OrbitTransactionStruct"
            }
          },
          {
            "name": "dataAddress",
            "type": "string"
          },
          {
            "name": "numKeys",
            "type": "u64"
          },
          {
            "name": "keyArr",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "finalDecision",
            "type": {
              "defined": "BuyerDecisionState"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "BuyerDecisionState",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Null"
          },
          {
            "name": "Declined"
          },
          {
            "name": "Accept"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "WrongDecisionAccount",
      "msg": "Wrong account to pay"
    },
    {
      "code": 6001,
      "name": "UndecidedTransaction",
      "msg": "Decision has not been made yet, can not close"
    },
    {
      "code": 6002,
      "name": "InvalidRateAcceptor",
      "msg": "Can not accept your own rate"
    },
    {
      "code": 6003,
      "name": "DidNotConfirmDelivery",
      "msg": "Please confirm delivery first"
    },
    {
      "code": 6004,
      "name": "WaitingForSellerData",
      "msg": "The seller did not commit keys yet"
    },
    {
      "code": 6005,
      "name": "InvalidSellerForListing",
      "msg": "The seller for the product does not match the seller given"
    },
    {
      "code": 6006,
      "name": "InvalidEscrowBump",
      "msg": "Could not compute escrow bump"
    },
    {
      "code": 6007,
      "name": "InvalidAuthBump",
      "msg": "Could not compute auth bump"
    },
    {
      "code": 6008,
      "name": "CorruptPrivateKeyFormat",
      "msg": "Could not decode private key"
    },
    {
      "code": 6009,
      "name": "IncorrectPrivateKey",
      "msg": "Private and Public keys do not match"
    },
    {
      "code": 6010,
      "name": "IndexOutOfRange",
      "msg": "Private and Public keys do not match"
    },
    {
      "code": 6011,
      "name": "CannotDiscountCommission",
      "msg": "Can not use discounts on commissions"
    },
    {
      "code": 6012,
      "name": "InvalidReflink",
      "msg": "invalid reflink passed"
    }
  ]
};
