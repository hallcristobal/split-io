mkdir -p responses
touch ./responses/game_search.json ./responses/game.json ./responses/game_runs.json ./responses/category.json ./responses/category_runs.json ./responses/user.json ./responses/user_runs.json ./responses/user_pbs.json ./responses/user_category_runs.json ./responses/user_category_pb.json ./responses/run.json

curl https://splits.io/api/v3/games?search=sonic > ./responses/game_search.json
curl https://splits.io/api/v3/games/sms > ./responses/game.json
curl https://splits.io/api/v3/games/sms/runs > ./responses/game_runs.json
curl https://splits.io/api/v3/games/sms/categories/852 > ./responses/category.json
curl https://splits.io/api/v3/games/sms/categories/70/runs > ./responses/category_runs.json
curl https://splits.io/api/v3/users/glacials > ./responses/user.json
curl https://splits.io/api/v3/users/glacials/runs > ./responses/user_runs.json
curl https://splits.io/api/v3/users/glacials/pbs > ./responses/user_pbs.json
curl https://splits.io/api/v3/users/glacials/games/sms/categories/852/runs > ./responses/user_category_runs.json
curl https://splits.io/api/v3/users/glacials/games/sms/categories/852/pb > ./responses/user_category_pb.json
curl https://splits.io/api/v3/runs/c6 > ./responses/run.json
# DELETE /runs/:id
# Deletes this run (requires authentication).
# 
# POST /runs
# Upload a new run. If you want to allow an account to claim ownership of this run, you should inspect the response for a uris.claim_uri and send the user who should own the run there. If they're logged in, their account will automatically claim ownership of the run, then they'll be immediately redirected to the uris.public_uri.
