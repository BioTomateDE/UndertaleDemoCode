
if hshake != 0 then
{
  if hshake < 0 then
       {
       view_xview[myview] += hshake
       hshake += 1
       }
       
  if hshake > 0 then
       {
       view_xview[myview] += hshake
       }
       hshake = -hshake
}

if vshake != 0 then
{
  if vshake > 0 then
       {
       view_yview[myview] += vshake
       }
  if vshake < 0 then
       {
       view_yview[myview] += vshake
       vshake += 1
       }
       vshake = -vshake
}
alarm[myview] = shakespeed

if hshake = 0 and vshake = 0 then instance_destroy()
