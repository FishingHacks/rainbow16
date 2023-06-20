--Geodezik
--by Fishi

function _init()
    frame = 0
end

function _update()
    frame = frame + 1
    if frame > 127 then
        frame = 0
    end
end

function _draw()
    cls()
    -- for i = 0,21 do
    --     local fe = frame * (i / 2)
    --     line(0, fe, 200-fe, 0, 7)
    --     line(fe, 180, 0, fe, 7)
    --     line(200, 180 - fe, fe, 180, 7)
    --     line(200 - fe, 0, 200, 180-fe, 7)
    -- end
    for i = 1,100 do
        print(i)
    end
end