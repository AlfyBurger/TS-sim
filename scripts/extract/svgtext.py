import sys,re,glob,os,json
import xml.etree.ElementTree as ET
out={}
for f in sorted(glob.glob(sys.argv[1]+'/TNRnTS-*.svg'), key=lambda x:int(re.findall(r'(\d+)\.svg',x)[0])):
    t=ET.parse(f).getroot()
    lines=[]
    for el in t.iter():
        if el.tag.endswith('text'):
            s=' '.join(''.join(x.itertext()).strip() for x in el if x.tag.endswith('tspan')) or ''.join(el.itertext())
            lines.append(s.strip())
    out[int(re.findall(r'(\d+)\.svg',f)[0])]=lines
json.dump(out,open(sys.argv[2],'w'),indent=1)
for k,v in out.items(): print(k,'|'.join(v)[:300])
