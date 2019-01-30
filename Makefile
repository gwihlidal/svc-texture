docker:
	docker build -t svc-texture .

container-build:
	gcloud container builds submit . --config=cloudbuild.yaml