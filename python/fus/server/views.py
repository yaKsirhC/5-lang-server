from django.http import  JsonResponse
import os
from .forms import UploadFileForm
from django.views.decorators.csrf import csrf_exempt
from django.http import FileResponse

dir_path = os.path.dirname(os.path.realpath(__file__))

def handle_uploaded_file(file):
    with open(f'uploads/{file.name}', 'wb+') as destination:
        for chunk in file.chunks():
            destination.write(chunk)

def sync(req):
    return JsonResponse({"files": os.listdir(os.path.join(dir_path, "..", "uploads"))})

@csrf_exempt
def upload(req):
    if req.method == "POST":
        form = UploadFileForm(req.POST, req.FILES)
        print(req.FILES["upload"])
        myfile = req.FILES['upload']
        handle_uploaded_file(myfile)
        return JsonResponse({"files": os.listdir(os.path.join(dir_path, "..", "uploads"))})

@csrf_exempt
def delete(req):
    if req.method == "DELETE":
        filename = req.GET.get("filename", "")
        os.remove(f"./uploads/{filename}")
        return JsonResponse({"files": os.listdir(os.path.join(dir_path, "..", "uploads"))})

def rtrv(req):
    filename = req.GET.get("filename", "")
    return FileResponse(open(f"./uploads/{filename}", 'rb'))