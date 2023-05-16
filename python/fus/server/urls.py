from django.urls import path
from . import views

urlpatterns = [
    path("sync", views.sync),
    path("upload-file", views.upload),
    path("delete", views.delete),
    path("retrieve", views.rtrv),
]