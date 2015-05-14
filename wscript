
#
# This file is the default set of rules to compile a Pebble project.
#
# Feel free to customize this to your needs.
#

from waflib import TaskGen

top = '.'
out = 'build'

TaskGen.declare_chain(
    name='rustc',
    rule='${RUSTC} -L ../lib/ --target ../platform/thumbv7m-none-eabi ${SRC} '
         '--emit=llvm-ir -A dead-code -o ${TGT}',
    ext_in=['.rs'],
    ext_out=['.ll'])

TaskGen.declare_chain(
    name='llc',
    rule=('${LLC} -mtriple=arm-none-eabi -relocation-model=pic -march=thumb '
          '-mattr=+thumb2 -mcpu=cortex-m3 --soft-float --asm-verbose=false '
          '-o ${TGT} ${SRC}'),
    ext_in=['.ll'],
    ext_out=['.s'])

TaskGen.declare_chain(
    name='as',
    rule='${AS} ${ASFLAGS} -c ${SRC} -o ${TGT}',
    ext_in=['.s'],
    ext_out=['.o'])

def options(ctx):
    ctx.load('pebble_sdk')

def configure(ctx):
    ctx.load('pebble_sdk')

def build(ctx):
    ctx.load('pebble_sdk')
    ctx.env.RUSTC = 'rustc'
    ctx.env.LLC = 'llc'
    
    ctx.pbl_program(source=ctx.path.ant_glob('src/*.(rs|c)'),
	            target='pebble-app.elf')

    ctx.pbl_bundle(elf="pebble-app.elf",
                    js=ctx.path.ant_glob('src/js/**/*.js'))
