function setup()
    my_print("boom time here we go")
end

function loop()
    fill(random(), random(), random())
    fillRect(random() * 300, random() * 300, random() * 200, random() * 200)
end

-- Epilogue; gather hooks and return
local out = {}
if type(setup)      == "function"  then out.setup   = setup;    end
if type(restore)    == "function"  then out.restore = restore;  end
if type(save)       == "function"  then out.save    = save;     end
if type(loop)       == "function"  then out.loop    = loop;     end
return out