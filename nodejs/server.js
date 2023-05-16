const express = require('express')
const fileupload = require('express-fileupload')
const fs = require('fs')
const path = require('path')
const app = express()

app.use(fileupload())

app.use('/', express.static('dist'))

app.get('/sync', (req, res) => {
    res.json({files: fs.readdirSync('./uploads')})
})

app.post('/upload-file', async (req, res) => {
    try {
        const upload = req.files.upload
        if(typeof upload[Symbol.iterator] == 'function'){
            for(const file of upload){
                console.log("Saving file: "+file.name)
                await file.mv('./uploads/'+file.name)
                
            }
        }else{
            console.log("Saving file: "+upload.name)
            await upload.mv('./uploads/'+upload.name)
        }
        res.json({files: fs.readdirSync('./uploads')})

    } catch (error) {
        console.error(error);
        res.sendStatus(500)
    }
})

app.delete('/delete', async (req,res) => {
    try {
        const filename = req.query.filename
        await fs.promises.rm('./uploads/'+filename)

        res.json({files: fs.readdirSync('./uploads')})
    } catch (error) {
        console.error(error);
        res.sendStatus(500)
    }
})

app.get('/retrieve', (req, res) => {
    const filename = req.query.filename
    res.sendFile(path.join(__dirname,'uploads',filename), {dotfiles: "allow"})
})

app.listen(9002)