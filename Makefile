PNG=$(wildcard video/*.png)
RAW=$(PNG:.png=.raw)

all: $(RAW)

video/%.raw: video/%.png
	embedded-mono-img -o $@ $^
