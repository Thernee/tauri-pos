-- CreateTable
CREATE TABLE "Customer" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL,
    "cardNumber" TEXT,
    "email" TEXT,
    "phone" TEXT,
    "balance" REAL NOT NULL DEFAULT 0.0,
    "totalDebt" REAL NOT NULL DEFAULT 0.0,
    "createdAt" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" DATETIME NOT NULL
);

-- CreateTable
CREATE TABLE "Transaction" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "customerId" INTEGER NOT NULL,
    "type" TEXT NOT NULL,
    "amount" REAL NOT NULL,
    "timestamp" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "note" TEXT,
    CONSTRAINT "Transaction_customerId_fkey" FOREIGN KEY ("customerId") REFERENCES "Customer" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "CashWallet" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "cashOnHand" REAL NOT NULL DEFAULT 0.0,
    "digitalWallet" REAL NOT NULL DEFAULT 0.0,
    "lastUpdated" DATETIME NOT NULL
);
