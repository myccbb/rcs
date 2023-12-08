import { PrismaClient as CenterClient, Prisma as Center } from "prisma_generated/center";

const center = new CenterClient({log: ['query']});

export { Center, center };
