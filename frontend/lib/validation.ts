export const sizes=["S","M","L","XL"]as const;
export function cleanText(v:unknown,max=120){return typeof v==="string"?v.trim().replace(/[<>]/g,"").slice(0,max):""}
export function cleanPhone(v:unknown){const x=typeof v==="string"?v.replace(/[^0-9+]/g,""):"";return /^\+?254\d{9}$/.test(x)?x:null}
export function positiveInt(v:unknown,max=1000){const n=Number(v);return Number.isInteger(n)&&n>0&&n<=max?n:null}
export function token(prefix:string){return prefix+crypto.randomUUID().replaceAll("-","").slice(0,10).toUpperCase()}
