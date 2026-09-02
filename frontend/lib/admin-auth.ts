import{headers}from"next/headers";
const OWNER_EMAIL="kiokojohn734@gmail.com";
export async function isOwner(){const h=await headers();return h.get("oai-authenticated-user-email")?.toLowerCase()===OWNER_EMAIL}
export async function requireOwnerApi(){if(!await isOwner())return Response.json({error:"Owner access required"},{status:403});return null}
